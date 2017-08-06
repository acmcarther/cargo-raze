#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate cargo;
extern crate rustc_serialize;
extern crate itertools;
extern crate tera;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod bazel;
mod planning;

use cargo::CargoError;
use cargo::CliResult;
use cargo::util::CargoResult;
use cargo::util::Cfg;
use cargo::util::Config;
use planning::PlannedDeps;
use planning::ResolvedPlan;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::ops::Deref;
use std::process::Command;
use std::str::FromStr;
use std::str;
use tera::Context;
use tera::Tera;

lazy_static! {
    pub static ref TERA: Tera = {
      let mut tera = Tera::new("src/not/a/dir/*").unwrap();
      tera.add_raw_templates(vec![
        ("templates/partials/rust_binary.template", include_str!("templates/partials/rust_binary.template")),
        ("templates/partials/rust_library.template", include_str!("templates/partials/rust_library.template")),
        ("templates/partials/rust_test.template", include_str!("templates/partials/rust_test.template")),
        ("templates/partials/rust_bench_test.template", include_str!("templates/partials/rust_bench_test.template")),
        ("templates/partials/rust_example.template", include_str!("templates/partials/rust_example.template")),
        ("templates/partials/build_script.template", include_str!("templates/partials/build_script.template")),
        ("templates/BUILD.template", include_str!("templates/BUILD.template"))]).unwrap();
      tera
    };
}

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

    if let Err(e) = result {
        cargo::exit_with_error(e, &mut *config.shell());
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
        .ok_or(CargoError::from("root crate should be in cargo resolve")));
    let root_direct_deps = resolve.deps(&root_package_id).cloned().collect::<HashSet<_>>();

    let targets_str = options.flag_targets.expect("--targets flag is mandatory: try 'x86_64-unknown-linux-gnu'");
    let platform_triple_env_pairs = targets_str
      .split(',')
      .map(|triple| {
        // Non lexical borrow dodge
        let attrs = fetch_attrs(triple);
        (triple, attrs)
      })
      .collect::<Vec<_>>();


    //let mut build_files = Vec::new();
    for (platform_triple, platform_attrs) in platform_triple_env_pairs.into_iter() {
      let mut crate_contexts = Vec::new();

      // TODO:(acmcarther): Reduce duplicate work: the next 20 lines are copy paste per platform
      for id in try!(planning::find_all_package_ids(options.flag_host.clone(), &config, &resolve)) {
          let package = packages.get(&id).unwrap().clone();
          let features = resolve.features(&id).clone();
          let full_name = format!("{}-{}", id.name(), id.version());
          let path = format!("./vendor/{}-{}/", id.name(), id.version());

          // Verify that package is really vendored
          try!(fs::metadata(&path).map_err(|_| {
              CargoError::from(format!("failed to find {}. Please run `cargo vendor -x` first.", &path))
          }));

          // Identify all possible dependencies
          let PlannedDeps { build_deps, dev_deps, normal_deps } =
              PlannedDeps::find_all_deps(&id, &package, &resolve, &platform_triple, &platform_attrs);

          let targets = try!(planning::identify_targets(&full_name, &package));
          let build_script_target = targets.iter().find(|t| t.kind.deref() == "custom-build").cloned();

          crate_contexts.push(bazel::CrateContext {
              pkg_name: id.name().to_owned(),
              pkg_version: id.version().to_string(),
              features: features,
              is_root_dependency: root_direct_deps.contains(&id),
              metadeps: Vec::new() /* TODO(acmcarther) */,
              dependencies: normal_deps,
              build_dependencies: build_deps,
              dev_dependencies: dev_deps,
              path: path,
              build_script_target: build_script_target,
              targets: targets,
              bazel_config: bazel::Config::default(),
              platform_triple: platform_triple.to_owned(),
          });
      }

      for package in &crate_contexts {
        //let crate_context = package.to_crate_context();
        let mut context = Context::new();
        context.add("crate", &package);
        context.add("path_prefix", &workspace_prefix);
        let rendered_file = TERA.render("templates/BUILD.template", &context).unwrap();

        let build_stub_path = format!("{}BUILD", &package.path);
        try!(File::create(&build_stub_path)
             .and_then(|mut f| f.write_all(rendered_file.as_bytes()))
             .map_err(|_| CargoError::from(format!("failed to create {}", build_stub_path))));
        println!("Generated {} successfully", build_stub_path);
      }
    }

    Ok(())
}

/** Verifies that the provided workspace_prefix is present and makes sense. */
fn validate_workspace_prefix(arg_buildprefix: Option<String>) -> CargoResult<String> {
    let workspace_prefix = try!(arg_buildprefix.ok_or(CargoError::from(
        "build prefix must be specified (in the form //path/where/vendor/is)")));

    if workspace_prefix.ends_with("/vendor") {
        return Err(CargoError::from(
            format!("Bazel path \"{}\" should not end with /vendor, you probably want \"{}\"",
                    workspace_prefix, workspace_prefix.chars().take(workspace_prefix.chars().count() - 7).collect::<String>())));
    }
    if workspace_prefix.ends_with("/") {
        return Err(CargoError::from(
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

