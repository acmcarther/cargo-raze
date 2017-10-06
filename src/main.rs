extern crate cargo as __internal_cargo_do_not_use;
#[macro_use(Deserialize)]
extern crate serde_derive;
extern crate serde;
extern crate itertools;
extern crate tera;
#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;
extern crate toml;
extern crate url;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

use std::env;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use log::LogLevel;
use util::OkButLog;
use url::Url;

mod _cargo {
  pub use __internal_cargo_do_not_use::CargoError as Error;
  pub use __internal_cargo_do_not_use::CliResult;
  pub use __internal_cargo_do_not_use::call_main_without_stdin;
  pub use __internal_cargo_do_not_use::core::Package;
  pub use __internal_cargo_do_not_use::core::PackageIdSpec;
  pub use __internal_cargo_do_not_use::core::Workspace;
  pub use __internal_cargo_do_not_use::core::Summary;
  pub use __internal_cargo_do_not_use::core::Dependency;
  pub use __internal_cargo_do_not_use::core::dependency::Kind;
  pub use __internal_cargo_do_not_use::core::registry::PackageRegistry;
  pub use __internal_cargo_do_not_use::core::registry::Registry;
  pub use __internal_cargo_do_not_use::exit_with_error;
  pub use __internal_cargo_do_not_use::core::resolver::Method;
  pub use __internal_cargo_do_not_use::core::resolver;
  pub use __internal_cargo_do_not_use::ops::resolve_with_previous;
  pub use __internal_cargo_do_not_use::util::CargoResult as CResult;
  pub use __internal_cargo_do_not_use::util::Config;
}

#[derive(Debug, Deserialize)]
struct Options {
    flag_verbose: u32,
    flag_host: Option<String>,
    flag_color: Option<String>,
    flag_target: Option<String>,
    flag_dryrun: Option<bool>,
    #[serde(rename = "flag_Z")]
    flag_z: Vec<String>,
}

const USAGE: &'static str = r#"
Generate BUILD files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze [options]

Options:
    -h, --help                Print this message
    -v, --verbose             Use verbose output
    --host HOST               Registry index to sync with
    --color WHEN              Coloring: auto, always, never
    --target TARGET           Platform to generate BUILD files for
    -d, --dryrun              Do not emit any files
    -Z FLAG ...                Unstable (nightly-only) flags to Cargo
"#;

fn main() {
  init::global_logger();
  let config = _cargo::Config::default().unwrap();
  let args = env::args().collect::<Vec<_>>();
  let result = _cargo::call_main_without_stdin(cargo_main, &config, USAGE, &args, false);

  if let Err(e) = result {
      _cargo::exit_with_error(e, &mut *config.shell());
  }
}

fn cargo_main(options: Options, config: &_cargo::Config) -> _cargo::CliResult {
  try!(config.configure(options.flag_verbose,
                        Some(false) /* quiet */,
                        &options.flag_color,
                        /* frozen = */ false,
                        /* locked = */ false,
                        &options.flag_z));

  let current_dir = env::current_dir().unwrap();

  InitialWorkspace::load_from_fs(&config, current_dir.join(&Path::new("Cargo.toml"))).ok_but_error();

  //debug!("{:#?}", ws);

  Ok(())
}


struct InitialWorkspace<'cfg> {
  manifest_path: PathBuf,
  raw_manifest: toml::Value,
  cargo_workspace: _cargo::Workspace<'cfg>,
}

impl<'cfg>  InitialWorkspace<'cfg> {
  fn load_from_fs(config: &_cargo::Config, manifest_path: PathBuf) -> _cargo::CResult<()> /*InitialWorkspace<'cfg>>*/ {
    let ws = try!(_cargo::Workspace::new(&manifest_path, config));
    let mut registry = PromotedDevDependencyRegistry {
      inner_registry: try!(_cargo::PackageRegistry::new(&ws.config()))
    };

    let mut summaries = Vec::new();

    for (url, patches) in ws.root_patch() {
        registry.patch(url, patches)?;
    }

    for member in ws.members() {
      let summary = registry.lock(member.summary().clone());
      summaries.push((summary, _cargo::Method::Everything));
    }

    debug!("hello buddy");

    let mut resolve = _cargo::resolver::resolve(
                                     &summaries /* summaries */,
                                     &[] /* replace */,
                                     &mut registry,
                                     Some(&ws.config()))?;
    debug!("where did my buddy go?");

    debug!("{:#?}", resolve);


    let manifest_contents =
      try!(util::load_file_forcefully(&manifest_path, "Cargo.toml in current working directory"));

    //debug!("{:#?}", specs);

    Ok(())

    /*
    InitialWorkspace {
      manifest_path: manifest_path
    }
    */
  }
}

mod init {
  use fern;
  use chrono;
  use log;
  use std;

  pub fn global_logger() {
    fern::Dispatch::new()
      .format(|out, message, record| {
          out.finish(format_args!("{} {} [{}] {}",
              record.level(),
              chrono::Local::now()
                  .format("%m%d %H:%M:%S%.6f"),
              record.target(),
              message))
      })
      .level(log::LogLevelFilter::Debug)
      .chain(std::io::stdout())
      .apply().unwrap();
  }
}

// TODO(acmcarther): Better Name
struct PromotedDevDependencyRegistry<'a> {
  inner_registry: _cargo::PackageRegistry<'a>,
}

impl <'a> PromotedDevDependencyRegistry<'a> {
  pub fn lock(&self, summary: _cargo::Summary) -> _cargo::Summary {
    self.inner_registry.lock(summary)
  }

  pub fn patch(&mut self, url: &Url, deps: &[_cargo::Dependency]) -> _cargo::CResult<()> {
    self.inner_registry.patch(url, deps)
  }
}

impl <'a> _cargo::Registry for PromotedDevDependencyRegistry<'a> {
  fn query(&mut self,
           dep: &_cargo::Dependency,
           f: &mut FnMut(_cargo::Summary)) -> _cargo::CResult<()> {
    self.inner_registry.query(
      dep,
      &mut |mut summary| {
        let summary = summary.map_dependencies(|mut dependency| {
          if dependency.kind() == _cargo::Kind::Development {
            dependency.set_kind(_cargo::Kind::Normal);
          }
          dependency
        });
        f(summary)
      })
  }

  fn query_vec(&mut self, dep: &_cargo::Dependency) -> _cargo::CResult<Vec<_cargo::Summary>> {
    let mut ret = Vec::new();
    self.query(dep, &mut |s| ret.push(s))?;
    Ok(ret)
  }

  fn supports_checksums(&self) -> bool {
    self.inner_registry.supports_checksums()
  }
}

mod util {
  use log::LogLevel;
  use std::fs::File;
  use std::path::PathBuf;
  use std::fmt::Display;
  use _cargo;

  pub fn load_file_forcefully(path: &PathBuf, file_description: &str) -> _cargo::CResult<String> {
    use std::io::Read;

    let mut c = String::new();
    try!(File::open(&path)
         .map_err(|_| "Failed to find")
         .and_then(|mut f| f.read_to_string(&mut c).map_err(|_| "Failed to load"))
         .map_err(|e| _cargo::Error::from(format!("{} {}", e, file_description))));
    trace!("Successfully loaded {}", file_description);
    Ok(c)
  }

  macro_rules! decl_ok_but {
    ($fn_ident:ident, $log_level:expr) => (
      fn $fn_ident(self) -> Option<T> {
        self.ok_but($log_level)
      }
    )
  }

  pub trait OkButLog<T> : Sized {
    fn ok_but(self, level: LogLevel) -> Option<T>;
    decl_ok_but!(ok_but_error, LogLevel::Error);
    decl_ok_but!(ok_but_warn, LogLevel::Warn);
    decl_ok_but!(ok_but_info, LogLevel::Info);
    decl_ok_but!(ok_but_debug, LogLevel::Debug);
    decl_ok_but!(ok_but_trace, LogLevel::Trace);
  }

  impl <T, U : Display> OkButLog<T> for Result<T, U> {
    fn ok_but(self, level: LogLevel) -> Option<T> {
      self.map_err(|e| log!(level, "{}", e)).ok()
    }
  }
}
