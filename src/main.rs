#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate cargo;
extern crate rustc_serialize;
extern crate itertools;
extern crate tera;
extern crate toml;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

mod context;
mod planning;
mod rendering;
mod settings;
mod util;
mod bazel;

use bazel::BazelRenderer;
use cargo::CargoError;
use cargo::CliResult;
use cargo::util::CargoResult;
use cargo::util::Config;
use planning::BuildPlanner;
use rendering::FileOutputs;
use rendering::BuildRenderer;
use rendering::RenderDetails;
use settings::RazeSettings;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;
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
Generate BUILD files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze

Options:
    -h, --help                Print this message
    -v, --verbose             Use verbose output
    --host HOST               Registry index to sync with
    -q, --quiet               No output printed to stdout
    --color WHEN              Coloring: auto, always, never
    -d, --dryrun              Do not emit any files
"#;

fn main() {
    let cargo_config = Config::default().unwrap();
    let args = env::args().collect::<Vec<_>>();
    let result = cargo::call_main_without_stdin(real_main, &cargo_config, USAGE, &args, false);

    if let Err(e) = result {
        cargo::exit_with_error(e, &mut *cargo_config.shell());
    }
}

fn real_main(options: Options, cargo_config: &Config) -> CliResult {
  try!(cargo_config.configure(options.flag_verbose,
                        options.flag_quiet,
                        &options.flag_color,
                        /* frozen = */ false,
                        /* locked = */ false));
  let mut settings = try!(load_settings("Cargo.toml"));
  println!("Loaded override settings: {:#?}", settings);

  try!(validate_settings(&mut settings));

  let mut planner = try!(BuildPlanner::new(settings, cargo_config));

  if let Some(host) = options.flag_host {
    try!(planner.set_registry_from_url(host));
  }


  let planned_build = try!(planner.plan_build());
  let mut bazel_renderer = BazelRenderer::new();
  let render_details = RenderDetails {
    path_prefix: "./".to_owned(),
  };

  let bazel_file_outputs = try!(bazel_renderer.render_planned_build(&render_details, &planned_build));

  let dry_run = options.flag_dryrun.unwrap_or(false);
  for FileOutputs { path, contents } in bazel_file_outputs {
    if !dry_run {
      try!(write_to_file_loudly(&path, &contents));
    } else {
      println!("{}:\n{}", path, contents);
    }
  }

  Ok(())
}

/** Verifies that the provided settings make sense. */
fn validate_settings(settings: &mut RazeSettings) -> CargoResult<()> {
  if !settings.vendor_path.starts_with("//") {
    return Err(CargoError::from("raze.vendor_path must start with \"//\". Paths into local repositories (such as @local//path) are currently unsupported."))
  }

  if settings.vendor_path.ends_with("/") && settings.vendor_path != "//" {
    settings.vendor_path.pop();
  }

  return Ok(())
}

fn write_to_file_loudly(path: &str, contents: &str) -> CargoResult<()> {
  try!(File::create(&path)
     .and_then(|mut f| f.write_all(contents.as_bytes()))
     .map_err(|_| CargoError::from(format!("failed to create {}", path))));
  println!("Generated {} successfully", path);
  Ok(())
}


fn load_settings<T: AsRef<Path>>(cargo_toml_path: T) -> Result<RazeSettings, CargoError> {
  let path = cargo_toml_path.as_ref();
  let mut toml = try!(File::open(path)
                      .map_err(|e| {
                        println!("{:?}", e);
                        CargoError::from(format!("Could not load {:?}", path))
                      }));
  let mut toml_contents = String::new();
  try!(toml.read_to_string(&mut toml_contents)
       .map_err(|e| {
         println!("{:?}", e);
         CargoError::from(format!("failed to read {:?}", path))
       }));
  toml::from_str::<settings::CargoToml>(&toml_contents)
    .map_err(|e| {
      println!("{:?}", e);
      CargoError::from(format!("failed to parse {:?}", path))
    })
    .map(|toml| toml.raze)
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
    // Vendoring into root is a really annoying edge case, not supported for now.
    assert!(validate_workspace_prefix(Some("//".to_owned())).is_err());
    assert!(validate_workspace_prefix(Some("//hello".to_owned())).is_ok());
  }
}
