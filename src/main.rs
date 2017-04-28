extern crate cargo;
#[macro_use]
extern crate nom;
extern crate rustc_serialize;

use cargo::CargoError;
use cargo::CliResult;
use cargo::core::Dependency;
use cargo::core::Package;
use cargo::core::PackageId;
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
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::str;

#[derive(Debug)]
pub struct DependencyWithPath {
  pub dependency: Dependency,
  pub path: String
}

/** Define parser for --overrides flag */
fn isnt_plus(chr: char) -> bool { chr != '+' }
fn isnt_colon(chr: char) -> bool { chr != ':' }
fn isnt_comma(chr: char) -> bool { chr != ',' }
named!(parse_override( &str ) -> DependencyOverride,
   do_parse!(
     name: map!(take_while_s!(isnt_plus), str::to_owned) >>
     char!('+') >>
     version: map!(take_while_s!(isnt_colon), str::to_owned) >>
     char!(':') >>
     bazel_path: map!(take_while_s!(isnt_comma), str::to_owned) >>
     (DependencyOverride { name: name, version: version, bazel_path: bazel_path })
   )
);
named!(parse_overrides( &str ) -> Vec<DependencyOverride>, separated_list!(char!(','), parse_override));

#[derive(Debug)]
pub struct RazePackage {
  pub id: PackageId,
  pub package: Package,
  pub features: HashSet<String>,
}


#[derive(RustcDecodable)]
struct Options {
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_host: Option<String>,
    flag_color: Option<String>,
    flag_overrides: Option<String>
}

#[derive(Debug)]
struct DependencyOverride {
    pub name: String,
    pub version: String,
    pub bazel_path: String,
}

const USAGE: &'static str = r#"
Generate BUILD files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze [options]

Options:
    -h, --help               Print this message
    -v, --verbose            Use verbose output
    --host HOST              Registry index to sync with
    -q, --quiet              No output printed to stdout
    --color WHEN             Coloring: auto, always, never
    --overrides LIST         Comma separated cargo dependency overrides ["libc+0.2.21:@workspace//path:dep,..."]
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
    let platform_triple = config.rustc()?.host.clone();
    let registry_id = try!(load_registry(options.flag_host, &config));
    try!(fs::metadata("Cargo.lock").chain_error(|| {
      human("failed to find Cargo.lock. Please run `cargo generate-lockfile` first.")
    }));
    let lockfile = Path::new("Cargo.lock");
    let manifest_path = lockfile.parent().unwrap().join("Cargo.toml");
    let manifest = env::current_dir().unwrap().join(&manifest_path);
    let ws = try!(Workspace::new(&manifest, config));
    let specs = Packages::All.into_package_id_specs(&ws).chain_error(|| {
      human("failed to find specs? whats a spec?")
    })?;

    // TODO(acmcarther): Fix unwrap. I'm unwrapping here temporarily because Nom's err is hard to
    // convert to CargoError
    let overrides = options.flag_overrides.as_ref()
      .map(|f| parse_overrides(f).to_result().unwrap())
      .unwrap_or(Vec::new());
    let override_name_and_ver_to_path: HashMap<(String, String), String> = overrides.into_iter()
      .map(|entry| ((entry.name, entry.version), entry.bazel_path))
      .collect();

    let (packages, resolve) = ops::resolve_ws_precisely(
            &ws,
            None,
            &[],
            false,
            false,
            &specs).chain_error(|| {
        human("failed to load pkg lockfile")
    })?;

    let mut package_ids = resolve.iter()
                     .filter(|id| *id.source_id() == registry_id)
                     .cloned()
                     .collect::<Vec<_>>();
    package_ids.sort_by_key(|id| id.name().to_owned());

    let mut max = HashMap::new();
    for id in package_ids.iter() {
        let max = max.entry(id.name()).or_insert(id.version());
        *max = cmp::max(id.version(), *max)
    }

    let mut raze_packages = Vec::new();

    for id in package_ids.iter() {
        // Skip generating new_crate_repository for overrides
        if override_name_and_ver_to_path.contains_key(&(id.name().to_owned(), id.version().to_string())) {
          continue
        }

        raze_packages.push(RazePackage {
          id: id.clone(),
          package: packages.get(id).unwrap().clone(),
          // TODO(acmcarther): This will break as of cargo commit 50f1c172
          features: resolve.features(id)
            .cloned()
            .unwrap_or(HashSet::new()),
        });
    }

    // Verify that cargo-vendor has already vendored the dependencies
    {
        for raze_pkg in raze_packages.iter() {
            let package_dir = format!("{}-{}/", raze_pkg.id.name(), raze_pkg.id.version());
            try!(fs::metadata(&package_dir).chain_error(|| {
                human(format!("failed to find {}. Please run `cargo vendor -x .` first.", package_dir))
            }));
        }
    }

    // Check for Build dependencies -- these will be lost on generation
    {
        let mut printed_warning = false;
        for raze_pkg in raze_packages.iter() {
            if raze_pkg.package.dependencies().iter().any(|dep| dep.kind() == Kind::Build) {
                printed_warning = true;
                println!("WARNING: Crate <{}-{}> appears to contain a Build dependency.",
                    raze_pkg.id.name(), raze_pkg.id.version());

            }
        }
        if printed_warning {
            println!("WARNING: You will probably need to override Build dependent crates with the --override flag and provide a custom BUILD rule.");
        }
    }
    {
        for raze_pkg in raze_packages.iter() {
            let &RazePackage {ref id, ref features, ref package, ..} = raze_pkg;
            let vendor_dir = format!("./{}-{}/", id.name(), id.version());
            let vendor_path = Path::new(&vendor_dir);
            let dependencies_by_name = package.dependencies().iter().cloned()
                .map(|dep| (dep.name().to_owned(), dep))
                .collect::<HashMap<String, Dependency>>();
            let dependencies = resolve.deps(id).into_iter()
              .map(|dep| {
                assert!(dependencies_by_name.contains_key(dep.name()));
                let overriid = override_name_and_ver_to_path.get(&(dep.name().to_owned(), dep.version().to_string()));
                let path = match overriid {
                  Some(override_path) => override_path.clone(),
                  None => format!("//{crate_full_name}:{sanitized_name}",
                    crate_full_name=format!("{}-{}", dep.name().replace("-", "_"), dep.version()),
                    sanitized_name=dep.name().replace("-", "_"))
                };
                DependencyWithPath {
                  dependency: dependencies_by_name.get(dep.name())
                    .expect("Dependencies from 'resolve' object should also be contained in 'package'")
                    .clone(),
                  path: path
                }
              })
              .collect::<Vec<_>>();
            let build_file_content = generate_build_file(
                                      id.name(),
                                      &dependencies,
                                      &features,
                                      &platform_triple);
            let build_file_path = vendor_path.join("BUILD");
            try!(File::create(&build_file_path).and_then(|mut f| f.write_all(build_file_content.as_bytes())).chain_error(|| {
                human(format!("failed to create: `{}`", build_file_path.display()))
            }));
        }
        let workspace_path = Path::new("WORKSPACE");
        try!(File::create(&workspace_path).chain_error(|| {
            human(format!("failed to create: `{}`", workspace_path.display()))
        }));
    }

    Ok(())
}

fn load_registry(flag_host: Option<String>, config: &Config) -> Result<SourceId, Box<CargoError>> {
    let source_id_from_registry =
      flag_host.map(|s| s.to_url().map(|url| SourceId::for_registry(&url)).map_err(human));

    source_id_from_registry.unwrap_or_else(|| SourceId::crates_io(config))
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


pub fn generate_build_file(crate_name: &str, dependencies: &Vec<DependencyWithPath>, feature_strings: &HashSet<String>, platform_triple: &str) -> String {
    let mut unused_dependencies_with_reason: Vec<String> = Vec::new();
    let mut dependency_strs = Vec::new();

    for dep in dependencies.iter() {
        let &DependencyWithPath { ref dependency, ref path } = dep;
        if dependency.kind() != Kind::Normal {
            unused_dependencies_with_reason.push(
              format!("Dependency <{}> is <{:?}> dependency", path.clone(), dependency.kind()));
            continue
        }

        if dependency.platform().map(|p| !p.matches(platform_triple, Some(&generic_linux_cfgs()))).unwrap_or(false) {
            unused_dependencies_with_reason.push(
              format!("Dependency <{}> is for alternate platform: <{}>", path.clone(), dependency.platform().unwrap()));
            continue
        }

        dependency_strs.push(path);
    }

    // Make output "stable" by sorting
    dependency_strs.sort();
    unused_dependencies_with_reason.sort();

    let unused_dependencies_str = unused_dependencies_with_reason.into_iter().map(|msg| format!("- {}\n", msg)).collect::<String>();
    format!(r#"'''
WARNING: THIS IS GENERATED CODE!
DO NOT MODIFY!
Instead, rerun raze with different arguments.

{notes}
'''
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "{sanitized_crate_name}",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
{comma_separated_deps}    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
{comma_separated_features}    ],
)"#,
    sanitized_crate_name = crate_name.replace("-", "_"),
    comma_separated_deps = dependency_strs.into_iter()
        .map(|dep_str| format!("        \"{}\",\n", dep_str))
        .collect::<String>(),
    comma_separated_features = bazelize_features(feature_strings),
    notes = format!(
"Unused dependencies from cargo: [
{unused_dependencies}]"
, unused_dependencies=unused_dependencies_str))
}

fn bazelize_features(features: &HashSet<String>) -> String {
    let mut feature_strs = features
      .iter()
      .map(|f| format!("        \"{}\",\n", f))
      .collect::<Vec<String>>();
    feature_strs.sort();
    return feature_strs.into_iter().collect::<String>();
}
