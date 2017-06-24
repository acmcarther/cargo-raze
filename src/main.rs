extern crate cargo;
extern crate rustc_serialize;

#[macro_use]
mod files;
mod bazel;

use files::BExpr;
use files::ToBExpr;
use cargo::CliResult;
use cargo::core::Dependency;
use cargo::core::Package;
use cargo::core::PackageId;
use cargo::core::TargetKind;
use cargo::core::SourceId;
use cargo::core::Workspace;
use cargo::core::dependency::Kind;
use cargo::ops::Packages;
use cargo::ops;
use cargo::util::Cfg;
use cargo::util::ChainError;
use cargo::util::Config;
use cargo::util::ToUrl;
use cargo::util::human;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::str;


#[derive(Debug, RustcDecodable)]
struct Options {
    arg_buildprefix: Option<String>,
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_host: Option<String>,
    flag_color: Option<String>,
    flag_overwrite: Option<bool>,
}

const USAGE: &'static str = r#"
Generate Cargo.bzl files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze [options] [<buildprefix>]

Options:
    -h, --help               Print this message
    -v, --verbose            Use verbose output
    --host HOST              Registry index to sync with
    -q, --quiet              No output printed to stdout
    --color WHEN             Coloring: auto, always, never
    --overwrite              Overwrite any customizable files (BUILD, CargoOverride.bzl)
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
    let workspace_prefix = options.arg_buildprefix

      .expect("build prefix must be specified (in the form //path/to/vendor/directory)");
    let platform_triple = config.rustc()?.host.clone();

    // TODO: use the fancy error chain stuff when I have time to grok it.
    assert!(
      workspace_prefix.ends_with("/vendor"),
      "workspace_prefix should end with /vendor (currently a hard limitation)");

    let (spec_escape, (packages, resolve)) = {
        let lockfile = Path::new("Cargo.lock");
        let manifest_path = lockfile.parent().unwrap().join("Cargo.toml");
        let manifest = env::current_dir().unwrap().join(&manifest_path);
        let ws = try!(Workspace::new(&manifest, config));
        let specs = Packages::All.into_package_id_specs(&ws).chain_error(|| {
          human("failed to find specs? whats a spec?")
        })?;
        (specs.clone(), ops::resolve_ws_precisely(
                &ws,
                None,
                &[],
                false,
                false,
                &specs).chain_error(|| {
            human("failed to load pkg lockfile")
        })?)
    };

    // TODO: clean this up -- it was the fastest way I could think to do this.
    let root_name = spec_escape.iter().next().unwrap().name().to_owned();
    let root_package_id = resolve.iter()
      .filter(|dep| dep.name() == root_name)
      .next()
      .expect("root crate should be in cargo resolve")
      .clone();
    let root_direct_deps = resolve.deps(&root_package_id).cloned().collect::<HashSet<_>>();

    let package_ids = {
        let source_id_from_registry =
          options.flag_host.map(|s| s.to_url().map(|url| SourceId::for_registry(&url)).map_err(human));

        let registry_id = try!(source_id_from_registry.unwrap_or_else(|| SourceId::crates_io(config)));
        try!(fs::metadata("Cargo.lock").chain_error(|| {
          human("failed to find Cargo.lock. Please run `cargo generate-lockfile` first.")
        }));

        let mut package_ids = resolve.iter()
            .filter(|id| *id.source_id() == registry_id)
            .cloned()
            .collect::<Vec<_>>();
        package_ids.sort_by_key(|id| id.name().to_owned());
        package_ids
    };

    let mut raze_packages = package_ids.iter()
      .map(|id| bazel::Package {
          id: id.clone(),
          package: packages.get(id).unwrap().clone(),
          full_name: format!("{}-{}", id.name(), id.version()),
          path: format!("./vendor/{}-{}/", id.name(), id.version()),
          // TODO(acmcarther): This will break as of cargo commit 50f1c172
          features: resolve.features(id)
            .cloned()
            .unwrap_or(HashSet::new()),
          is_root_dependency: root_direct_deps.contains(id),
          dependencies: Vec::new(),
          build_dependencies: Vec::new(),
          dev_dependencies: Vec::new(),
          targets: Vec::new(),
          bazel_config: bazel::Config::default(),
      })
      .collect::<Vec<_>>();

    // Verify that the package is already vendored
    for pkg in raze_packages.iter() {
        try!(fs::metadata(&pkg.path).chain_error(|| {
            human(format!("failed to find {}. Please run `cargo vendor -x` first.", pkg.path))
        }));
    }

    // Determine targets
    for mut pkg in raze_packages.iter_mut() {
        let &mut bazel::Package {
          ref full_name,
          ref package,
          ref mut targets, ..} = pkg;
        let partial_path = format!("{}/", full_name);
        let partial_path_byte_length = partial_path.as_bytes().len();

        for target in package.targets().iter() {
            let target_path_str = target.src_path().to_str()
              .expect("path wasn't unicode")
              .to_owned();
            let crate_name_str_idx = target_path_str.find(&partial_path)
              .expect("target path should have been in vendor directory");
            let local_path_bytes = target_path_str.bytes()
              .skip(crate_name_str_idx + partial_path_byte_length)
              .collect::<Vec<_>>();
            let local_path = String::from_utf8(local_path_bytes)
              .expect("source string was corrupted while slicing");

            targets.push(bazel::Target {
              name: target.name().to_owned(),
              path: local_path,
              kinds: kind_to_kinds(target.kind()),
            });
        }
    }

    let platform_attrs = generic_linux_cfgs();

    // Determine exact dependencies per package
    for mut pkg in raze_packages.iter_mut() {
        let &mut bazel::Package {
          ref id,
          ref package,
          ref mut dependencies,
          ref mut build_dependencies,
          ref mut dev_dependencies, ..} = pkg;

        let concrete_dependencies = package.dependencies().iter()
            .filter(|dep| dep.platform().map(|p| p.matches(&platform_triple, Some(&platform_attrs))).unwrap_or(true))
            .cloned()
            .collect::<Vec<_>>();

        let normal_dependencies_by_name = concrete_dependencies.iter()
            .filter(|dep| dep.kind() == Kind::Normal)
            .map(|dep| (dep.name().to_owned(), dep.clone()))
            .collect::<HashMap<String, Dependency>>();

        let dev_dependencies_by_name = concrete_dependencies.iter()
            .filter(|dep| dep.kind() == Kind::Development)
            .map(|dep| (dep.name().to_owned(), dep.clone()))
            .collect::<HashMap<String, Dependency>>();

        let build_dependencies_by_name = concrete_dependencies.iter()
            .filter(|dep| dep.kind() == Kind::Build)
            .map(|dep| (dep.name().to_owned(), dep.clone()))
            .collect::<HashMap<String, Dependency>>();

        let planned_dependencies_by_name = resolve.deps(id).into_iter()
            .map(|dep| (dep.name().to_owned(), dep.clone()))
            .collect::<HashMap<String, PackageId>>();

        let all_dependency_names = planned_dependencies_by_name.keys().cloned()
            .chain(normal_dependencies_by_name.keys().cloned())
            .chain(dev_dependencies_by_name.keys().cloned())
            .chain(build_dependencies_by_name.keys().cloned())
            .collect::<HashSet<_>>();

        for dependency_name in all_dependency_names.iter() {
            if !planned_dependencies_by_name.contains_key(dependency_name) {
                // TODO(acmcarther): Identify why this is removing most dev dependencies
                //println!("TRACE: Crate <{}> is omitting concrete dependency <{}> because it is unused.",
                //         id.name(), dependency_name);
                continue
            }
            let planned_dependency = planned_dependencies_by_name.get(dependency_name).unwrap();
            let bazel_dependency = bazel::Dependency {
                name: dependency_name.clone(),
                version: planned_dependency.version().to_string(),
            };

            if let Some(_) = dev_dependencies_by_name.get(dependency_name) {
                dev_dependencies.push(bazel_dependency.clone());
            }

            if let Some(_) = build_dependencies_by_name.get(dependency_name) {
                build_dependencies.push(bazel_dependency.clone());
            }

            if let Some(_) = normal_dependencies_by_name.get(dependency_name) {
                dependencies.push(bazel_dependency);
            }
        }
    }

    let workspace = bazel::Workspace::new(&raze_packages, &platform_triple, &platform_attrs, &workspace_prefix);

    try!(files::generate_override_file(options.flag_overwrite.unwrap_or(false)));
    try!(files::generate_workspace_file(&workspace));

    Ok(())
}

fn generic_linux_cfgs() -> Vec<Cfg> {
    // TODO(acmcarther): use output of rustc, similar to
    // cargo::ops::cargo_rustc::context::Context::probe_target_info_kind
    // https://github.com/rust-lang/cargo/blob/f5348cc321a032db95cd18e3129a4392d2e0a892/src/cargo/ops/cargo_rustc/context.rs#L199
    let hardcoded_properties =
r#"debug_assertions
target_arch="x86_64"
target_endian="little"
target_env="gnu"
target_family="unix"
target_feature="sse"
target_feature="sse2"
target_has_atomic="16"
target_has_atomic="32"
target_has_atomic="64"
target_has_atomic="8"
target_has_atomic="ptr"
target_os="linux"
target_pointer_width="64"
target_thread_local
target_vendor="unknown"
unix"#;
    hardcoded_properties.lines()
      .map(Cfg::from_str)
      .map(|cfg| cfg.expect("hardcoded values should be parsable"))
      .collect()
}

fn cfg_pretty(cfg: &Cfg) -> String {
    match cfg {
        &Cfg::Name(ref s) => s.clone(),
        &Cfg::KeyPair(ref k, ref v) => format!("{}: {}", k, v)
    }
}

// TODO(acmcarther): Remove this shim from cargo when Cargo is upgraded
fn kind_to_kinds(kind: &TargetKind) -> Vec<String> {
    match kind {
        &TargetKind::Lib(ref kinds) => kinds.iter().map(|k| k.crate_type().to_owned()).collect(),
        &TargetKind::Bin => vec!["bin".to_owned()],
        &TargetKind::ExampleBin | &TargetKind::ExampleLib(_) => vec!["example".to_owned()],
        &TargetKind::Test => vec!["test".to_owned()],
        &TargetKind::CustomBuild => vec!["custom-build".to_owned()],
        &TargetKind::Bench => vec!["bench".to_owned()],
    }
}
