extern crate cargo;
extern crate rustc_serialize;

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
use std::iter;
use std::hash::Hash;
use std::cmp::Eq;
use std::fs;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::str;

// A basic expr type for bzl files
pub enum BExpr {
  Value(String),
  Array(Vec<BExpr>),
  Struct(Vec<(String, BExpr)>),
}

impl BExpr {
  pub fn pretty_print(&self) -> String {
    self.pretty_print_spaced(4 /* space_count */)
  }

  fn pretty_print_spaced(&self, space_count: usize) -> String {
    assert!(space_count >= 4);
    let less_spaces = iter::repeat(' ')
      .take(if space_count > 0 { space_count - 4 } else { 0 })
      .collect::<String>();
    let spaces = iter::repeat(' ')
      .take(space_count)
      .collect::<String>();
    match self {
      &BExpr::Value(ref s) => format!("\"{}\"", s),
      &BExpr::Array(ref a) if a.len() == 0 => format!("[]"),
      &BExpr::Array(ref a) => format!("[\n{}{}]", a.iter()
                              .map(|i| format!("{}{},\n", spaces, i.pretty_print_spaced(space_count + 4)))
                              .collect::<String>(), less_spaces),
      &BExpr::Struct(ref s) if s.len() == 0 => format!("struct()"),
      &BExpr::Struct(ref s) => format!("struct(\n{}{})", s.iter()
                              .map(|&(ref k, ref v)| format!("{}{} = {},\n", spaces, k, v.pretty_print_spaced(space_count + 4)))
                              .collect::<String>(), less_spaces),
    }
  }
}

// Produces a hashmap-ish Struct BExpr
macro_rules! b_struct {
    ($($key:expr => $value:expr),*) => {
        {
            let mut contents: Vec<(String, BExpr)> = Vec::new();
            $(
              contents.push(($key.to_string(), $value));
            )*
            BExpr::Struct(contents)
        }
    };
}

// Produces an array-ish BExpr
macro_rules! b_array {
    ($($value:expr),*) => {
        {
            let mut contents: Vec<BExpr> = Vec::new();
            $(
              contents.push($value);
            )*
            BExpr::Array(contents)
        }
    };
}

// Produces a string-ish value BExpr
macro_rules! b_value {
    ($value:expr) => {
      BExpr::Value($value.to_string())
    };
}

trait ToBExpr {
  fn to_expr(&self) -> BExpr;
}

impl <T> ToBExpr for Vec<T> where T: ToBExpr {
  fn to_expr(&self) -> BExpr {
    BExpr::Array(self.iter().map(|v| v.to_expr()).collect())
  }
}
impl <T> ToBExpr for HashSet<T> where T: Eq + Hash + ToBExpr {
  fn to_expr(&self) -> BExpr {
    BExpr::Array(self.iter().map(|v| v.to_expr()).collect())
  }
}

impl <T> ToBExpr for HashMap<String, T> where T: ToBExpr {
  fn to_expr(&self) -> BExpr {
    BExpr::Struct(self.iter().map(|(k, v)| (k.clone(), v.to_expr())).collect())
  }
}

impl ToBExpr for String {
  fn to_expr(&self) -> BExpr {
    b_value!(self)
  }
}

#[derive(Debug, Clone)]
pub struct BazelPackage {
  pub id: PackageId,
  pub package: Package,
  pub features: HashSet<String>,
  pub full_name: String,
  pub path: String,
  pub dependencies: Vec<BazelDependency>,
  pub build_dependencies: Vec<BazelDependency>,
  pub dev_dependencies: Vec<BazelDependency>,
  pub targets: Vec<BazelTarget>,
}

impl ToBExpr for BazelPackage {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "package" => b_struct! {
        "pkg_name" => b_value!(self.id.name()),
        "pkg_version" => b_value!(self.id.version())
      },
      "dependencies" => self.dependencies.to_expr(),
      "build_dependencies" => self.build_dependencies.to_expr(),
      "dev_dependencies" => self.dev_dependencies.to_expr(),
      "features" => self.features.to_expr(),
      "targets" => self.targets.to_expr()
    }
  }
}

#[derive(Debug, Clone)]
pub struct BazelDependency {
  pub name: String,
  pub version: String,
}

impl ToBExpr for BazelDependency {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "name" => b_value!(self.name),
      "version" => b_value!(self.version)
    }
  }
}

#[derive(Debug, Clone)]
pub struct BazelTarget {
  pub name: String,
  pub kinds: Vec<String>,
  pub path: String,
}

impl ToBExpr for BazelTarget {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "name" => b_value!(self.name),
      "kinds" => self.kinds.to_expr(),
      "path" => b_value!(self.path)
    }
  }
}

#[derive(RustcDecodable)]
struct Options {
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_host: Option<String>,
    flag_color: Option<String>,
}

const USAGE: &'static str = r#"
Generate Cargo.bzl files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze [options]

Options:
    -h, --help               Print this message
    -v, --verbose            Use verbose output
    --host HOST              Registry index to sync with
    -q, --quiet              No output printed to stdout
    --color WHEN             Coloring: auto, always, never
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

    let mut raze_packages = package_ids.iter()
      .map(|id| BazelPackage {
          id: id.clone(),
          package: packages.get(id).unwrap().clone(),
          full_name: format!("{}-{}", id.name(), id.version()),
          path: format!("./vendor/{}-{}/", id.name(), id.version()),
          // TODO(acmcarther): This will break as of cargo commit 50f1c172
          features: resolve.features(id)
            .cloned()
            .unwrap_or(HashSet::new()),
          dependencies: Vec::new(),
          build_dependencies: Vec::new(),
          dev_dependencies: Vec::new(),
          targets: Vec::new(),
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
        let &mut BazelPackage {
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

            targets.push(BazelTarget {
              name: target.name().to_owned(),
              path: local_path,
              kinds: kind_to_kinds(target.kind()),
            });
        }
    }

    let platform_attrs = generic_linux_cfgs();

    // Determine exact dependencies per package
    for mut pkg in raze_packages.iter_mut() {
        let &mut BazelPackage {
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
            let bazel_dependency = BazelDependency {
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

    let platform_attrs_pretty = platform_attrs.iter().map(cfg_pretty).collect::<Vec<_>>();

    for pkg in raze_packages.into_iter() {
        let file_contents = format!(
r#""""
cargo-raze generated details for {name}.

Generated for:
platform_triple: {platform_triple}
platform_attrs:
{platform_attrs:#?}

DO NOT MODIFY! Instead, add a CargoOverride.bzl mixin.
"""
description = {expr}
"#,
            name = pkg.full_name,
            platform_triple = platform_triple,
            platform_attrs = platform_attrs_pretty,
            expr = pkg.to_expr().pretty_print());

        let cargo_bzl_path = format!("{}Cargo.bzl", &pkg.path);
        try!(File::create(&cargo_bzl_path)
             .and_then(|mut f| f.write_all(file_contents.as_bytes()))
             .chain_error(|| human(format!("failed to create {}", cargo_bzl_path))));
        println!("Generated {} successfully", cargo_bzl_path);
    }

    println!("All done!");
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
