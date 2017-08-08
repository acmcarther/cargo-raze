#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate cargo;
extern crate rustc_serialize;
extern crate itertools;
extern crate tera;

mod context;
mod planning;
mod rendering;
mod util;

use cargo::CargoError;
use cargo::CliResult;
use cargo::util::CargoResult;
use cargo::util::Config;
use planning::BuildPlanner;
use planning::FileOutputs;
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Debug, RustcDecodable)]
struct Options {
    arg_buildprefix: Option<String>,
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_host: Option<String>,
    flag_color: Option<String>,
    flag_target: Option<String>,
    flag_dryrun: Option<bool>,
}

const USAGE: &'static str = r#"
Generate Bazel BUILD files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze [<buildprefix>] [options]

Options:
    -h, --help                Print this message
    -v, --verbose             Use verbose output
    --host HOST               Registry index to sync with
    -q, --quiet               No output printed to stdout
    --color WHEN              Coloring: auto, always, never
    --target TARGET           Platform to generate BUILD files for
    -d, --dryrun              Do not emit any files
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
  let mut planner = try!(BuildPlanner::new(
    try!(validate_workspace_prefix(options.arg_buildprefix)),
    options.flag_target.expect("--target flag is mandatory: try 'x86_64-unknown-linux-gnu'"),
    config,
  ));

  if let Some(host) = options.flag_host {
    try!(planner.set_registry_from_url(host));
  }

  let planned_build = try!(planner.plan_build());
  let file_outputs = try!(planned_build.render());

  let dry_run = options.flag_dryrun.unwrap_or(false);
  for FileOutputs { path, contents } in file_outputs {
    if !dry_run {
      try!(write_to_file_loudly(&path, &contents));
    } else {
      println!("{}:\n{}", path, contents);
    }
  }

  Ok(())
}

/** Verifies that the provided workspace_prefix is present and makes sense. */
fn validate_workspace_prefix(arg_buildprefix: Option<String>) -> CargoResult<String> {
    let workspace_prefix = try!(arg_buildprefix.ok_or(CargoError::from(
        "build prefix must be specified (in the form //path/where/vendor/is)")));

    if !(workspace_prefix.starts_with("//") || workspace_prefix.starts_with("@")) {
        return Err(CargoError::from(
            format!("Bazel path \"{}\" should begin with \"//\" or \"@\"", workspace_prefix)));
    }

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

fn write_to_file_loudly(path: &str, contents: &str) -> CargoResult<()> {
  try!(File::create(&path)
     .and_then(|mut f| f.write_all(contents.as_bytes()))
     .map_err(|_| CargoError::from(format!("failed to create {}", path))));
  println!("Generated {} successfully", path);
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_prefix_expects_some_value() {
    assert!(validate_workspace_prefix(None).is_err());
  }

  #[test]
  fn test_validate_prefix_expects_initial_slashes_or_at() {
    assert!(validate_workspace_prefix(Some("hello".to_owned())).is_err());
    assert!(validate_workspace_prefix(Some("@hello".to_owned())).is_ok());
    assert!(validate_workspace_prefix(Some("//hello".to_owned())).is_ok());
  }

  #[test]
  fn test_validate_prefix_expects_no_vendor() {
    assert!(validate_workspace_prefix(Some("//hello/vendor".to_owned())).is_err());
    assert!(validate_workspace_prefix(Some("//hello".to_owned())).is_ok());
  }

  #[test]
  fn test_validate_prefix_expects_no_slash() {
    assert!(validate_workspace_prefix(Some("//hello/".to_owned())).is_err());
    assert!(validate_workspace_prefix(Some("//hello".to_owned())).is_ok());
  }
}
