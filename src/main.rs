extern crate cargo;
extern crate rustc_serialize;
extern crate itertools;

#[macro_use]
mod files;
mod bazel;
mod planning;

use cargo::CargoError;
use cargo::CliResult;
use cargo::util::Cfg;
use cargo::util::ChainError;
use cargo::util::Config;
use cargo::util::human;
use std::collections::HashSet;
use std::process::Command;
use std::env;
use std::fs;
use std::str::FromStr;
use std::str;
use planning::ResolvedPlan;
use planning::PlannedDeps;
use itertools::Itertools;

#[derive(Debug, RustcDecodable)]
struct Options {
    arg_buildprefix: Option<String>,
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_host: Option<String>,
    flag_color: Option<String>,
    flag_targets: Option<String>,
}

const USAGE: &'static str = r#"
Generate Bazel BUILD files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze [options] [<buildprefix>]

Options:
    -h, --help                Print this message
    -v, --verbose             Use verbose output
    --host HOST               Registry index to sync with
    -q, --quiet               No output printed to stdout
    --color WHEN              Coloring: auto, always, never
    --targets TARGETS         List of comma-separated target triples to generate BUILD for
"#;

fn main() {
    let config = Config::default().unwrap();
    let args = env::args().collect::<Vec<_>>();
    let result = cargo::call_main_without_stdin(real_main, &config, USAGE, &args, false);

    match result {
        Err(e) => cargo::handle_cli_error(e, &mut *config.shell()),
        Ok(()) => {},
    }
}

fn real_main(options: Options, config: &Config) -> CliResult {
    try!(config.configure(options.flag_verbose,
                          options.flag_quiet,
                          &options.flag_color,
                          /* frozen = */ false,
                          /* locked = */ false));
    let workspace_prefix = try!(validate_workspace_prefix(options.arg_buildprefix));

    let ResolvedPlan {root_name, packages, resolve} =
        try!(ResolvedPlan::resolve_from_files(&config));

    let root_package_id = try!(resolve.iter()
        .filter(|dep| dep.name() == root_name)
        .next()
        .ok_or(human("root crate should be in cargo resolve")));
    let root_direct_deps = resolve.deps(&root_package_id).cloned().collect::<HashSet<_>>();

    let targets_str = options.flag_targets.expect("--targets flag is mandatory");
    let platform_triple_env_pairs = targets_str
      .split(',')
      .map(|triple| {
        // Non lexical borrow dodge
        let attrs = fetch_attrs(triple);
        (triple, attrs)
      })
      .collect::<Vec<_>>();


    let mut build_files = Vec::new();
    for (platform_triple, platform_attrs) in platform_triple_env_pairs.into_iter() {
      let mut raze_packages = Vec::new();

      // TODO:(acmcarther): Reduce duplicate work: the next 20 lines are copy paste per platform
      for id in try!(planning::find_all_package_ids(options.flag_host.clone(), &config, &resolve)) {
          let package = packages.get(&id).unwrap().clone();
          let features = resolve.features(&id).cloned().unwrap_or(HashSet::new());
          let full_name = format!("{}-{}", id.name(), id.version());
          let path = format!("./vendor/{}-{}/", id.name(), id.version());

          // Verify that package is really vendored
          try!(fs::metadata(&path).chain_error(|| {
              human(format!("failed to find {}. Please run `cargo vendor -x` first.", &path))
          }));

          // Identify all possible dependencies
          let PlannedDeps { build_deps, dev_deps, normal_deps } =
              PlannedDeps::find_all_deps(&id, &package, &resolve, &platform_triple, &platform_attrs);

          raze_packages.push(bazel::Package {
              features: features,
              is_root_dependency: root_direct_deps.contains(&id),
              metadeps: Vec::new() /* TODO(acmcarther) */,
              dependencies: normal_deps,
              build_dependencies: build_deps,
              dev_dependencies: dev_deps,
              path: path,
              targets: try!(planning::identify_targets(&full_name, &package)),
              bazel_config: bazel::Config::default(),
              id: id,
              package: package,
              full_name: full_name,
          });
      }

      for package in &raze_packages {
        //try!(files::generate_crate_bzl_file(&package));
        build_files.push(files::generate_crate_build_file(&package, &platform_triple, &workspace_prefix));
      }
    }

    build_files.sort_by(|a, b| a.get_path().cmp(b.get_path()));
    let unique_files = build_files
      .into_iter()
      .group_by(|a| a.get_path().to_owned())
      .into_iter()
      .map(|(_, mut fs)| {
        let first = fs.next().unwrap();
        fs.fold(first, |mut a, b| {
          a.merge_with_file(b);
          a
        })
      })
      .collect::<Vec<_>>();

    for file in unique_files {
      try!(file.write_self())
    }

    //let workspace = bazel::Workspace::new(&raze_packages, &platform_triple, &platform_attrs);

    // TODO(acmcarther): Support this, somehow... Right now theres not an obvious choice for
    // platform
    //try!(files::generate_vendor_build_file(&raze_packages, &workspace_prefix));
    //try!(files::generate_workspace_bzl_file(&workspace));

    // TODO(acmcarther): Remove, override is unsupported
    //try!(files::generate_override_bzl_file(false));
    //try!(files::generate_outer_build_file());

    Ok(())
}

/** Verifies that the provided workspace_prefix is present and makes sense. */
fn validate_workspace_prefix(arg_buildprefix: Option<String>) -> Result<String, Box<CargoError>> {
    let workspace_prefix = try!(arg_buildprefix.ok_or(human(
        "build prefix must be specified (in the form //path/where/vendor/is)")));

    if workspace_prefix.ends_with("/vendor") {
        return Err(human(
            format!("Bazel path \"{}\" should not end with /vendor, you probably want \"{}\"",
                    workspace_prefix, workspace_prefix.chars().take(workspace_prefix.chars().count() - 7).collect::<String>())));
    }
    if workspace_prefix.ends_with("/") {
        return Err(human(
            format!("Bazel path \"{}\" should not end with /, you probably want \"{}\"",
                    workspace_prefix, workspace_prefix.chars().take(workspace_prefix.chars().count() - 1).collect::<String>())));
    }

    Ok(workspace_prefix)
}


/**
 * Gets the proper system attributes for the provided platform triple using rustc.
 */
fn fetch_attrs(target: &str) -> Vec<Cfg> {
    let args = vec![
      format!("--target={}", target),
      "--print=cfg".to_owned(),
    ];
    let output = Command::new("rustc")
        .args(&args)
        .output()
        .expect(&format!("could not run rustc to fetch attrs for target {}", target));

    if !output.status.success() {
      panic!(format!("getting target attrs for {} failed with status: '{}' \n\
                     stdout: {}\n\
                     stderr: {}",
                     target,
                     output.status,
                     String::from_utf8(output.stdout).unwrap_or("[unparseable bytes]".to_owned()),
                     String::from_utf8(output.stderr).unwrap_or("[unparseable bytes]".to_owned())))
    }

    let attr_str = String::from_utf8(output.stdout)
        .expect("successful run of rustc's output to be utf8");

    attr_str.lines()
        .map(Cfg::from_str)
        .map(|cfg| cfg.expect("attrs from rustc should be parsable into Cargo Cfg"))
        .collect()
}

