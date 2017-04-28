extern crate cargo;
#[macro_use]
extern crate nom;
extern crate rustc_serialize;
extern crate itertools;

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
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::str;

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
    // TODO(acmcarther): Fix unwrap. I'm unwrapping here temporarily because Nom's err is hard to
    // convert to CargoError
    let override_name_and_ver_to_path: HashMap<(String, String), String> = {
        options.flag_overrides.as_ref()
            .map(|f| parse_overrides(f).to_result().unwrap())
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|entry| ((entry.name, entry.version), entry.bazel_path))
            .collect()
    };

    let (packages, resolve) = {
        let lockfile = Path::new("Cargo.lock");
        let manifest_path = lockfile.parent().unwrap().join("Cargo.toml");
        let manifest = env::current_dir().unwrap().join(&manifest_path);
        let ws = try!(Workspace::new(&manifest, config));
        let specs = Packages::All.into_package_id_specs(&ws).chain_error(|| {
          human("failed to find specs? whats a spec?")
        })?;

        ops::resolve_ws_precisely(
                &ws,
                None,
                &[],
                false,
                false,
                &specs).chain_error(|| {
            human("failed to load pkg lockfile")
        })?
    };

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

    let raze_packages = package_ids.iter()
      .filter(|id| !override_name_and_ver_to_path.contains_key(&(id.name().to_owned(), id.version().to_string())))
      .map(|id| RazePackage {
          id: id.clone(),
          package: packages.get(id).unwrap().clone(),
          // TODO(acmcarther): This will break as of cargo commit 50f1c172
          features: resolve.features(id)
            .cloned()
            .unwrap_or(HashSet::new())})
      .collect::<Vec<_>>();

    for pkg in raze_packages.iter() {
        let &RazePackage {ref id, ref features, ref package, ..} = pkg;
        let vendor_dir = format!("./{}-{}/", id.name(), id.version());

        try!(fs::metadata(&vendor_dir).chain_error(|| {
            human(format!("failed to find {}. Please run `cargo vendor -x .` first.", vendor_dir))
        }));

        let vendor_path = Path::new(&vendor_dir);

        if package.dependencies().iter().any(|dep| dep.kind() == Kind::Build) {
            println!("WARNING: Crate <{}-{}> appears to contain a Build dependency.",
                id.name(), id.version());
        }

        let package_dependencies_by_name = package.dependencies().iter().cloned()
            .map(|dep| (dep.name().to_owned(), dep))
            .collect::<HashMap<String, Dependency>>();

        let resolved_dependencies_by_name = resolve.deps(id).into_iter()
            .map(|dep| (dep.name().to_owned(), dep.clone()))
            .collect::<HashMap<String, PackageId>>();

        let all_dependencies = package_dependencies_by_name.keys().cloned()
            .chain(resolved_dependencies_by_name.keys().cloned())
            .collect::<HashSet<_>>();

        let mut bazel_dependency_strs = Vec::new();
        for dependency_name in all_dependencies.iter() {
            if !resolved_dependencies_by_name.contains_key(dependency_name) {
                continue
            }
            let resolved_dependency = resolved_dependencies_by_name.get(dependency_name).unwrap();
            assert!(package_dependencies_by_name.contains_key(dependency_name));
            let package_dependency = package_dependencies_by_name.get(dependency_name).unwrap();

            let dependency_version = resolved_dependency.version();
            let dependency_override = override_name_and_ver_to_path
                .get(&(dependency_name.to_owned(), dependency_version.to_string()));

            if package_dependency.kind() != Kind::Normal {
                println!("WARNING: Crate <{}> is dropping <{:?}> dependency <{}>",
                         id.name(), package_dependency.kind(), dependency_name);
                continue
            }

            let platform_requires_dependency = package_dependency.platform()
              .map(|p| p.matches(&platform_triple, Some(&generic_linux_cfgs())))
              .unwrap_or(true);
            if platform_requires_dependency {
                println!("INFO: Crate <{}> is dropping <{}> because it is not for this plaform.",
                         id.name(), dependency_name);
                println!("INFO: Dependency <{}>'s target specification is <{:?}>",
                         dependency_name, package_dependency.platform().unwrap());
                continue
            }

            let bazel_dependency_path = match dependency_override {
                Some(override_path) => override_path.clone(),
                None => format!("//{sanitized_name}-{crate_version}:{sanitized_name}",
                  sanitized_name=dependency_name.replace("-", "_"),
                  crate_version=resolved_dependency.version())
            };
            bazel_dependency_strs.push(bazel_dependency_path);
        }

        let sanitized_crate_name = id.name().replace("-", "_");
        let mut comma_separated_deps = bazel_dependency_strs.into_iter()
            .map(|dep_str| format!("        \"{}\",\n", dep_str))
            .collect::<Vec<String>>();
        comma_separated_deps.sort();
        let mut comma_separated_features = features
            .iter()
            .map(|f| format!("        \"{}\",\n", f))
            .collect::<Vec<String>>();
        comma_separated_features.sort();

        let comma_separated_deps_str =
            comma_separated_deps.into_iter().collect::<String>();
        let comma_separated_features_str =
            comma_separated_features.into_iter().collect::<String>();

        let build_file_content = generate_build_file(
            &sanitized_crate_name,
            &comma_separated_deps_str,
            &comma_separated_features_str);

        let build_file_path = vendor_path.join("BUILD");
        try!(File::create(&build_file_path)
            .and_then(|mut f| f.write_all(build_file_content.as_bytes()))
            .chain_error(|| human(format!("failed to create: `{}`", build_file_path.display()))));
    }
    let workspace_path = Path::new("WORKSPACE");
    try!(File::create(&workspace_path)
        .chain_error(|| human(format!("failed to create: `{}`", workspace_path.display()))));

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


pub fn generate_build_file(
        sanitized_crate_name: &str,
        comma_separated_deps: &String,
        comma_separated_features: &String) -> String {
    format!(r#"'''
WARNING: THIS IS GENERATED CODE!
DO NOT MODIFY!
Instead, rerun raze with different arguments.
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
        sanitized_crate_name=sanitized_crate_name,
        comma_separated_deps=comma_separated_deps,
        comma_separated_features=comma_separated_features)
}
