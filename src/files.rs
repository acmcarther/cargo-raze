use bazel;
use cargo::util::ChainError;
use cargo::CargoError;
use cargo::util::human;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::fs::File;
use std::str;
use std::iter;
use std::hash::Hash;
use std::cmp::Eq;
use bazel::ExampleValue;

// A basic expr type for bzl files
// TODO(acmcarther): Tuples
pub enum BExpr {
  Bool(bool),
  Optional(Option<Box<BExpr>>),
  Str(String),
  Tuple(Vec<BExpr>),
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
      &BExpr::Bool(true) => "True".to_owned(),
      &BExpr::Bool(false) => "False".to_owned(),
      &BExpr::Optional(None) => "None".to_owned(),
      &BExpr::Optional(Some(ref a)) => a.pretty_print_spaced(space_count),
      &BExpr::Str(ref s) => format!("\"{}\"", s),
      &BExpr::Tuple(ref a) if a.len() == 0 => format!("()"),
      &BExpr::Tuple(ref a) => format!("(\n{}{})", a.iter()
                              .map(|i| format!("{}{},\n", spaces, i.pretty_print_spaced(space_count + 4)))
                              .collect::<String>(), less_spaces),
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
    BExpr::Str($value.to_string())
  };
}

pub trait ToBExpr {
  fn to_expr(&self) -> BExpr;
}

impl <T> ToBExpr for Option<T> where T: ToBExpr {
  fn to_expr(&self) -> BExpr {
    BExpr::Optional(self.as_ref().map(|v| Box::new(v.to_expr())))
  }
}

impl <T> ToBExpr for Vec<T> where T: ToBExpr + Ord {
  fn to_expr(&self) -> BExpr {
    let mut array_items: Vec<&T> = self.iter().collect();
    array_items.sort();
    BExpr::Array(array_items.iter().map(|v| v.to_expr()).collect())
  }
}
impl <T> ToBExpr for HashSet<T> where T: Eq + Hash + ToBExpr + Ord {
  fn to_expr(&self) -> BExpr {
    let mut array_items: Vec<&T> = self.iter().collect();
    array_items.sort();
    BExpr::Array(array_items.iter().map(|v| v.to_expr()).collect())
  }
}

impl <T> ToBExpr for HashMap<String, T> where T: ToBExpr {
  fn to_expr(&self) -> BExpr {
    let mut map_keys: Vec<String> = self.keys().cloned().collect();
    map_keys.sort();
    BExpr::Struct(map_keys.into_iter().map(|k| {
      let map_val = self.get(&k).unwrap().to_expr();
      (k, map_val)
    }).collect())
  }
}

impl <T, U> ToBExpr for (T, U) where T: Ord + ToBExpr, U: ToBExpr {
  fn to_expr(&self) -> BExpr {
    BExpr::Tuple(vec![self.0.to_expr(), self.1.to_expr()])
  }
}

impl ToBExpr for String {
  fn to_expr(&self) -> BExpr {
    b_value!(self)
  }
}

impl ToBExpr for bool {
  fn to_expr(&self) -> BExpr {
    BExpr::Bool(*self)
  }
}

pub fn generate_crate_bzl_file(pkg: &bazel::Package) -> Result<(), Box<CargoError>> {
   let file_contents = format!(
r#""""
cargo-raze generated details for {name}.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = {expr}
"#,
        name = pkg.full_name,
        expr = pkg.to_crate_config().to_expr().pretty_print());

    let cargo_bzl_path = format!("{}Crate.bzl", &pkg.path);
    try!(File::create(&cargo_bzl_path)
         .and_then(|mut f| f.write_all(file_contents.as_bytes()))
         .chain_error(|| human(format!("failed to create {}", cargo_bzl_path))));
    println!("Generated {} successfully", cargo_bzl_path);
    Ok(())
}

pub fn generate_crate_build_file(pkg: &bazel::Package,
                                 platform_triple: &str,
                                 workspace_prefix: &str) -> bazel::BuildFile {
    let path = format!("{}BUILD", &pkg.path);
    let prelude = format!(
r#""""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["{workspace_prefix}:__subpackages__"])
"#, workspace_prefix = workspace_prefix);

    let mut load_statements = HashSet::new();
    load_statements.insert(r#"load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")"#.to_owned());


    let crate_name_sanitized = pkg.id.name().to_owned().replace("-", "_");
    let mut build_script_target = None;
    let mut lib_target = None;
    for target in pkg.targets.clone() {
      if target.kinds.contains(&"lib".to_owned()) {
        if lib_target.is_some() {
          panic!("package {} had more than one 'lib' type target!", crate_name_sanitized)
        }
        lib_target= Some(target.clone())
      }

      if target.kinds.contains(&"custom-build".to_owned()) {
        if build_script_target.is_some() {
          panic!("package {} had more than one 'custom-build' type target!", crate_name_sanitized)
        }
        build_script_target = Some(target.clone())
      }

      // TODO(acmcarther): Support other build types (plugins, binaries)
    }

    let platform_triple_sanitized = platform_triple.to_owned().replace("-", "_");
    let mut build_rules = Vec::new();
    let mut features_sorted = pkg.features.iter().map(|f| format!("\"{}\"", f)).collect::<Vec<_>>();
    features_sorted.sort();
    // TODO: deduplicate "normal_deps" and "build_deps"
    let normal_deps = {
      let mut out = Vec::new();
      for dep in pkg.dependencies.clone() {
        let sanitized_dependency_name = dep.name.to_owned().replace("-", "_");
        out.push(format!("\"{workspace_prefix}/vendor/{dependency_name}-{dependency_version}:{sanitized_dependency_name}_{platform_triple_sanitized}\"",
                workspace_prefix = workspace_prefix,
                dependency_name = dep.name,
                dependency_version = dep.version,
                platform_triple_sanitized = platform_triple_sanitized,
                sanitized_dependency_name = sanitized_dependency_name));
      }
      out
    };
    let build_deps = {
      let mut out = Vec::new();
      for dep in pkg.build_dependencies.clone() {
        let sanitized_dependency_name = dep.name.to_owned().replace("-", "_");
        out.push(format!("\"{workspace_prefix}/vendor/{dependency_name}-{dependency_version}:{sanitized_dependency_name}_{platform_triple_sanitized}\"",
                workspace_prefix = workspace_prefix,
                dependency_name = dep.name,
                dependency_version = dep.version,
                platform_triple_sanitized = platform_triple_sanitized,
                sanitized_dependency_name = sanitized_dependency_name));
      }
      out
    };

    if let Some(ref real_build_script_target) = build_script_target {

      let mut build_deps_complete = build_deps.iter().chain(normal_deps.iter()).cloned().collect::<Vec<_>>();
      build_deps_complete.sort();
      let binary_rule = format!(
r#"rust_binary(
    name = {crate_name_sanitized}_{platform_triple_sanitized}_build_script
    srcs = glob(["*", "src/**/*.rs"]),
    crate_root = "{real_build_script_target_path}",
    deps = [{deps}],
    rustc_flags = [
        "--cap-lints allow",
        "--target={platform_triple}"
    ],
    crate_features = [{crate_features}],
)"#,
          crate_name_sanitized = crate_name_sanitized,
          platform_triple_sanitized = platform_triple_sanitized,
          real_build_script_target_path = real_build_script_target.path,
          deps = build_deps_complete.join(", "),
          crate_features = features_sorted.join(", "),
          platform_triple = platform_triple);

      let workspace_prefix_sans_initial_slashes = workspace_prefix.chars().skip(2).collect::<String>();

      let genrule_rule = format!(
r#"genrule(
    name = {crate_name_sanitized}_{platform_triple_sanitized}_build_script_executor
    srcs = glob(["*", "src/**/*.rs"]),
    outs = ["{crate_name_sanitized}_{platform_triple_sanitized}_out_dir_outputs.tar.gz"],
    tools = [":{crate_name_sanitized}_{platform_triple_sanitized}_build_script"],
    cmd = "mkdir {crate_name_sanitized}_{platform_triple_sanitized}_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/{workspace_prefix_sans_initial_slashes}/vendor/{crate_name}-{crate_version}\";"
        + " export TARGET='{platform_triple}';"
        + " export RUST_BACKTRACE=1;"
        + " export OUT_DIR=$$PWD/{crate_name_sanitized}_{platform_triple_sanitized}_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :{crate_name_sanitized}_{platform_triple_sanitized}_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)"#,
          crate_name_sanitized = crate_name_sanitized,
          crate_name = pkg.id.name().to_owned(),
          crate_version = pkg.id.version().to_string(),
          platform_triple_sanitized = platform_triple_sanitized,
          workspace_prefix_sans_initial_slashes = workspace_prefix_sans_initial_slashes,
          platform_triple = platform_triple);

      build_rules.push(binary_rule);
      build_rules.push(genrule_rule);
    }

    if let Some(real_lib_target) = lib_target {
      let target_name_sanitized = real_lib_target.name.to_owned().replace("-", "_");
      if target_name_sanitized != crate_name_sanitized {
        let crate_name_alias_to_lib_rule = format!(
r#"alias(
    name = "{crate_name_sanitized}_{platform_triple_sanitized}",
    actual = ":{target_name_sanitized}_{platform_triple_sanitized}",
)"#, 
           crate_name_sanitized = crate_name_sanitized,
           platform_triple_sanitized = platform_triple_sanitized,
           target_name_sanitized = target_name_sanitized);
        build_rules.push(crate_name_alias_to_lib_rule);
      }

      let out_dir_tar = if build_script_target.is_some() {
        format!("\":{crate_name_sanitized}_{platform_triple_sanitized}_build_script_executor\"",
                crate_name_sanitized = crate_name_sanitized,
                platform_triple_sanitized = platform_triple_sanitized)
      } else {
        "None".to_owned()
      };

      let mut deps_sorted = normal_deps.iter().cloned().collect::<Vec<_>>();
      deps_sorted.sort();
      let library_rule = format!(
r#"rust_library(
    name = "{target_name_sanitized}_{platform_triple_sanitized}",
    crate_root = "{target_path}",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [{deps}],
    rustc_flags = [
        "--cap-lints allow",
        "--target={platform_triple}"
    ],
    out_dir_tar = {out_dir_tar},
    crate_features = [{crate_features}],
)"#,
          target_name_sanitized = target_name_sanitized,
          platform_triple_sanitized = platform_triple_sanitized,
          target_path = real_lib_target.path,
          crate_features = features_sorted.join(", "),
          deps = deps_sorted.join(", "),
          out_dir_tar = out_dir_tar,
          platform_triple = platform_triple);
      build_rules.push(library_rule);
    }

    bazel::BuildFile::new(path, prelude, load_statements, build_rules)
}

pub fn generate_vendor_build_file(raze_packages: &Vec<bazel::Package>,
                                  workspace_prefix: &str) -> Result<(), Box<CargoError>> {
    let aliases = raze_packages.iter()
      .filter(|pkg| pkg.is_root_dependency)
      .map(|pkg| format!(
r#"
alias(
    name = "{name}",
    actual = "{workspace_prefix}/vendor/{full_name}:{sanitized_name}",
)
"#, name = pkg.id.name(), sanitized_name = pkg.id.name().replace("-", "_"), workspace_prefix = workspace_prefix, full_name = pkg.full_name))
      .collect::<String>();
    let file_contents = format!(
r#""""
cargo-raze direct Cargo.toml dependencies.

This BUILD file provides aliases to explicit cargo dependencies and is
the only way to access vendored dependencies.
If a dependency is missing, add it as an explicit root dependency in
Cargo.toml and rerun raze.
This file is overridden on runs of raze; do not add anything to it.
If that is causing you pain, please drop a line in acmcarther/cargo-raze.
"""
package(default_visibility = ["//visibility:public"])
{aliases}
"#, aliases = aliases);
    let alias_file_path = "./vendor/BUILD";
    try!(File::create(alias_file_path)
         .and_then(|mut f| f.write_all(file_contents.as_bytes()))
         .chain_error(|| human(format!("failed to create {}", alias_file_path))));
    println!("Generated {} successfully", alias_file_path);
    Ok(())
}

pub fn generate_outer_build_file() -> Result<(), Box<CargoError>> {
    let outer_build_file_path = format!("./BUILD");
    if !fs::metadata(&outer_build_file_path).is_ok() {
      try!(File::create(&outer_build_file_path)
           .chain_error(|| human(format!("failed to create {}", outer_build_file_path))));
      println!("Generated {} successfully", outer_build_file_path);
    } else {
      println!("Skipping BUILD, since it already exists.");
    }

    Ok(())
}

pub fn generate_override_bzl_file(should_overwrite: bool) -> Result<(), Box<CargoError>> {
    let file_contents = format!(
r#""""
cargo-raze vendor-wide override file

Make your changes here. Bazel automatically integrates overrides from this
file and will not overwrite it on a rerun of cargo-raze.

Properties defined here will take priority over generated properties.

Reruns of cargo-raze may change the versions of your dependencies. Fear not!
cargo-raze will warn you if it detects an override for different version of a
dependency, to prompt you to update the specified override version.
"""
override_cfg = {override_cfg}
"#, override_cfg = bazel::OverrideSettings::example_value().to_expr().pretty_print());
    let cargo_override_bzl_path = format!("./CargoOverrides.bzl");
    if should_overwrite || !fs::metadata(&cargo_override_bzl_path).is_ok() {
      try!(File::create(&cargo_override_bzl_path)
           .and_then(|mut f| f.write_all(file_contents.as_bytes()))
           .chain_error(|| human(format!("failed to create {}", cargo_override_bzl_path))));
      println!("Generated {} successfully", cargo_override_bzl_path);
    } else {
      println!("Skipping CargoOverrides.bzl, since it already exists.");
    }

    Ok(())
}

pub fn generate_workspace_bzl_file(workspace: &bazel::Workspace) -> Result<(), Box<CargoError>> {
    let file_contents = format!(
r#""""
cargo-raze vendor-wide workspace file

DO NOT EDIT! Replaced on runs of cargo-raze
"""

workspace = {expr}
"#, expr = workspace.to_expr().pretty_print());
    let workspace_file_path = format!("./Cargo.bzl");
    try!(File::create(&workspace_file_path)
         .and_then(|mut f| f.write_all(file_contents.as_bytes()))
         .chain_error(|| human(format!("failed to create {}", workspace_file_path))));
    println!("Generated {} successfully", workspace_file_path);
    Ok(())
}
