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

// A basic expr type for bzl files
// TODO(acmcarther): Tuples
pub enum BExpr {
  Bool(bool),
  Value(String),
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
      &BExpr::Value(ref s) => format!("\"{}\"", s),
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
    BExpr::Value($value.to_string())
  };
}

pub trait ToBExpr {
  fn to_expr(&self) -> BExpr;
}

impl <T> ToBExpr for Vec<T> where T: ToBExpr {
  fn to_expr(&self) -> BExpr {
    BExpr::Array(self.iter().map(|v| v.to_expr()).collect())
  }
}
impl <T> ToBExpr for HashSet<T> where T: Eq + Hash + ToBExpr {
  fn to_expr(&self) -> BExpr {
    // TODO(acmcarther): This is not stable, and will generate diffs when nothing changes
    BExpr::Array(self.iter().map(|v| v.to_expr()).collect())
  }
}

impl <T> ToBExpr for HashMap<String, T> where T: ToBExpr {
  fn to_expr(&self) -> BExpr {
    BExpr::Struct(self.iter().map(|(k, v)| (k.clone(), v.to_expr())).collect())
  }
}

impl <T, U> ToBExpr for (T, U) where T: ToBExpr, U: ToBExpr {
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
        expr = pkg.to_expr().pretty_print());

    let cargo_bzl_path = format!("{}Crate.bzl", &pkg.path);
    try!(File::create(&cargo_bzl_path)
         .and_then(|mut f| f.write_all(file_contents.as_bytes()))
         .chain_error(|| human(format!("failed to create {}", cargo_bzl_path))));
    println!("Generated {} successfully", cargo_bzl_path);
    Ok(())
}

pub fn generate_crate_build_file(pkg: &bazel::Package,
                                 workspace_prefix: &str) -> Result<(), Box<CargoError>> {
    let build_stub_contents = format!(
r#""""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["{workspace_prefix}:__subpackages__"])

load("@io_bazel_rules_raze//raze:raze.bzl", "cargo_library")
load(":Crate.bzl", "description")
load("{workspace_prefix}:Cargo.bzl", "workspace")
load("{workspace_prefix}:CargoOverrides.bzl", "overrides")

cargo_library(
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    crate_bzl = description,
    cargo_override_bzl = overrides,
    platform = workspace.platform,
    workspace_path = "{workspace_prefix}/"
)
"#, workspace_prefix = workspace_prefix);
    let build_stub_path = format!("{}BUILD", &pkg.path);
    try!(File::create(&build_stub_path)
         .and_then(|mut f| f.write_all(build_stub_contents.as_bytes()))
         .chain_error(|| human(format!("failed to create {}", build_stub_path))));
    println!("Generated {} successfully", build_stub_path);
    Ok(())
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

pub fn generate_outer_build_file(should_overwrite: bool) -> Result<(), Box<CargoError>> {
    let outer_build_file_path = format!("./BUILD");
    if should_overwrite || !fs::metadata(&outer_build_file_path).is_ok() {
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

Override entries should be of identical form to generated Crate.bzl entries.
Properties defined here will take priority over generated properties.

Reruns of cargo-raze may change the versions of your dependencies. Fear not!
cargo-raze will warn you if it detects an override for different version of a
dependency, to prompt you to update the specified override version.
"""
overrides = []
"#,);
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
