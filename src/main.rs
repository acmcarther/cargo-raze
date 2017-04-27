extern crate cargo;
#[macro_use]
extern crate nom;
extern crate rustc_serialize;

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::str;
use std::path::Path;

use cargo::CargoError;
use cargo::ops::Packages;
use cargo::core::dependency::Kind;
use cargo::core::SourceId;
use cargo::core::Workspace;
use cargo::ops;
use cargo::CliResult;
use cargo::util::{human, ChainError, ToUrl, Config};

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


#[derive(RustcDecodable)]
struct Options {
    flag_verbose: u32,
    flag_quiet: Option<bool>,
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
    // TODO(acmcarther): Stub
    Ok(())
}
