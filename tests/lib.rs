// TODO(acmcarther): Do this, if they every libify this plugin
// extern crate cargo_vendor
extern crate cargo;
extern crate tempdir;

use tempdir::TempDir;
use std::io::Write;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::process::Command;
use std::env;

fn generate_cargo_toml(deps: &Vec<(&str, &str)>) -> String {
  let dependency_entries = deps.iter()
    .map(|&(name, ver)| format!("\"{}\" = \"{}\"", name, ver))
    .collect::<Vec<_>>();
  let dependency_str = dependency_entries.join("\n");
  format!(
r#"[package]
name = "compile_with_bazel"
version = "0.1.0"

[dependencies]
{dependencies}

[lib]
path = "fake_lib.rs"
"#, dependencies = dependency_str)
}

fn generate_bazel_workspace() -> String {
  format!(
r#"
# For examples/ dir
git_repository(
    name = "io_bazel_rules_rust",
    remote = "https://github.com/acmcarther/rules_rust.git",
    commit = "c9b9c1a"
)
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()
"#)
}

fn generate_dummy_lib_rs(dep_names: &Vec<String>) -> String {
  dep_names.iter()
    .map(|name| format!("extern crate {};\n", name.replace("-", "_")))
    .collect()
}

fn generate_dummy_build(dep_names: &Vec<String>) -> String {
  let dep_bazel_paths = dep_names.iter()
    .map(|name| format!("\"//cargo/vendor:{}\",\n", name.replace("-", "_")))
    .collect::<String>();

  format!(
r#"
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
  name = "dummy",
  srcs = ["lib.rs"],
  deps = [{deps}]
)"#, deps = dep_bazel_paths)
}

fn run_generate_lockfile(path: &Path) {
  let output = Command::new("cargo")
    .args(&["generate-lockfile"])
    .current_dir(path)
    .output()
    .expect("could not run generate-lockfile");
  println!("cargo generate-lockfile => stdout: {}", String::from_utf8(output.stdout).unwrap());
  println!("cargo generate-lockfile => stderr: {}", String::from_utf8(output.stderr).unwrap());
}

fn run_cargo_vendor(path: &Path) {
  let output = Command::new("cargo")
    .args(&["vendor", "-x"])
    .current_dir(path)
    .output()
    .expect("could not run cargo vendor");
  println!("cargo vendor => stdout: {}", String::from_utf8(output.stdout).unwrap());
  println!("cargo vendor => stderr: {}", String::from_utf8(output.stderr).unwrap());
}

fn run_cargo_raze(path: &Path, vendor_path: &str) {
  let current_exe = env::current_exe().unwrap();
  let own_bin = current_exe.parent().unwrap().parent().unwrap()
    .join(format!("cargo-raze{}", env::consts::EXE_SUFFIX));
  let target_flag = format!("--target={}", "x86_64-unknown-linux-gnu");
  let output = Command::new(own_bin)
    .args(&["raze", vendor_path, &target_flag])
    .current_dir(path)
    .output()
    .expect("could not run cargo raze");
  println!("cargo raze => stdout: {}", String::from_utf8(output.stdout).unwrap());
  println!("cargo raze => stderr: {}", String::from_utf8(output.stderr).unwrap());
}

struct TestScenario {
  path_prefix: String,
  temp_dir: TempDir,
  build_command: Command,
}

impl TestScenario {
  pub fn from_parts(path_prefix: String, temp_dir: TempDir, build_command: Command) -> TestScenario {
    TestScenario {
      path_prefix: path_prefix,
      temp_dir: temp_dir,
      build_command: build_command,
    }
  }

  pub fn generate_with_deps(deps: Vec<(&str, &str)>) -> TestScenario {
    let temp_dir = TempDir::new("cargo_raze_test").expect("Could not generate tempdir");
    let mut files = Vec::new();
    let raze_dir = temp_dir.as_ref().join("cargo/");
    fs::create_dir(&raze_dir).expect("couldn't create raze dir");

    let cargo_toml_path = raze_dir.join("Cargo.toml");
    let cargo_toml_contents = generate_cargo_toml(&deps);
    files.push((cargo_toml_path, cargo_toml_contents));

    let bazel_workspace_path = temp_dir.as_ref().join("WORKSPACE");
    let bazel_workspace_contents = generate_bazel_workspace();
    files.push((bazel_workspace_path, bazel_workspace_contents));

    let dep_names = deps.iter().map(|&(name, _)| name.to_owned()).collect::<Vec<_>>();
    files.push((temp_dir.as_ref().join("lib.rs"), generate_dummy_lib_rs(&dep_names)));
    files.push((temp_dir.as_ref().join("BUILD"), generate_dummy_build(&dep_names)));


    for (path, contents) in files {
      File::create(&path)
         .and_then(|mut f| f.write_all(contents.as_bytes()))
         .expect("could not create file");
    }

    run_generate_lockfile(&raze_dir);
    run_cargo_vendor(&raze_dir);
    run_cargo_raze(&raze_dir, "//cargo");

    let output = Command::new("tree")
      .current_dir(temp_dir.as_ref())
      .output()
      .expect("could not run tree");
    println!("tree => stdout: {}", String::from_utf8(output.stdout).unwrap());
    println!("tree => stderr: {}", String::from_utf8(output.stderr).unwrap());

    let mut build_command = Command::new("bazel");

    build_command
        .args(&["build", "//:dummy"])
        .current_dir(temp_dir.as_ref());

    TestScenario::from_parts("//".to_owned(), temp_dir, build_command)
  }

  pub fn get_command(&mut self) -> &mut Command {
    &mut self.build_command
  }
}

fn get_scenarios() -> Vec<TestScenario> {
  vec![
    // Basic Example
    TestScenario::generate_with_deps(vec![("lazy_static", "=0.2.8")]),
    // Fetching transitive dependencies
    TestScenario::generate_with_deps(vec![("memchr", "=1.0.1")]),
    // Possessing common dependencies
    TestScenario::generate_with_deps(vec![("libc", "=0.2.29"), ("aho-corasick", "=0.6.3")]),
    // TODO(acmcarther): Identify an example crate that generates .rs files, but does NOT need
    // stdout configuration support (i.e. "cargo:rustc-cfg=feature=\"some_feature\"")
  ]
}

#[test]
fn scenarios_build_successfully() {
  for mut scenario in get_scenarios() {
    let output = scenario.get_command().output().expect("could not run scenario command");
    let stderr = String::from_utf8(output.stderr).unwrap();
    println!("stdout: {}", String::from_utf8(output.stdout).unwrap());
    println!("stderr: {}", stderr);

    assert!(stderr.contains("INFO: Build completed successfully"));
  }
}
