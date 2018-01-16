use log::LogLevel;
use std::fs::File;
use std::path::PathBuf;
use std::fmt::Display;
use std::collections::HashSet;
use std::process::Command;
use std::str::FromStr;
use std::str;
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

pub fn fetch_attrs(target: &str) -> _cargo::CResult<Vec<_cargo::Cfg>> {
    let args = vec![
      format!("--target={}", target),
      "--print=cfg".to_owned(),
    ];


    let output = try!(Command::new("rustc")
        .args(&args)
        .output()
        .map_err(|_| CargoError::from(format!("could not run rustc to fetch attrs for target {}", target))));

    if !output.status.success() {
      panic!(format!("getting target attrs for {} failed with status: '{}' \n\
                     stdout: {}\n\
                     stderr: {}",
                     target,
                     output.status,
                     String::from_utf8(output.stdout).unwrap_or("[unparseable bytes]".to_owned()),
                     String::from_utf8(output.stderr).unwrap_or("[unparseable bytes]".to_owned())))
    }

    let attr_str = String::from_utf8(output.stdout)
        .expect("successful run of rustc's output to be utf8");

    Ok(attr_str.lines()
        .map(Cfg::from_str)
        .map(|cfg| cfg.expect("attrs from rustc should be parsable into Cargo Cfg"))
        .collect())
}
