extern crate cargo;
extern crate rustc_serialize;
#[macro_use]
extern crate derive_builder;

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
use std::env;
use std::fs;
use std::str::FromStr;
use std::str;
use planning::ResolvedPlan;
use planning::PlannedDeps;

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
    --overwrite              Overwrite CargoOverride.bzl
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

    let platform_attrs = generic_linux_cfgs();
    let platform_triple = config.rustc()?.host.clone();

    let mut raze_packages = Vec::new();
    for id in try!(planning::find_all_package_ids(options.flag_host, &config, &resolve)) {
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
      try!(files::generate_crate_bzl_file(&package));
      try!(files::generate_crate_build_file(&package, &workspace_prefix));
    }

    let workspace = bazel::Workspace::new(&raze_packages, &platform_triple, &platform_attrs);

    try!(files::generate_vendor_build_file(&raze_packages, &workspace_prefix));
    try!(files::generate_workspace_bzl_file(&workspace));
    try!(files::generate_override_bzl_file(options.flag_overwrite.unwrap_or(false)));
    try!(files::generate_outer_build_file());

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
 * Generates an artificial set of Cfg objects based on a standard linux system.
 *
 * This function is meant to be temporary. It should be replaced by proper platform introspection
 * similar to Cargo's own:
 * cargo::ops::cargo_rustc::context::Context::probe_target_info_kind
 * https://github.com/rust-lang/cargo/blob/f5348cc321a032db95cd18e3129a4392d2e0a892/src/cargo/ops/cargo_rustc/context.rs#L199
 */
fn generic_linux_cfgs() -> Vec<Cfg> {
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

