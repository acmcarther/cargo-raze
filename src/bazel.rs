use files::BExpr;
use files::ToBExpr;
use cargo::util::Cfg as CargoCfg;
use cargo::core::Package as CargoPackage;
use std::collections::HashSet;
use cargo::core::PackageId;

#[derive(Debug, Clone)]
pub struct Config {
  pub use_build_rs: bool,
  pub use_metadeps: bool
}

impl Default for Config {
  fn default() -> Config {
    Config {
      use_build_rs: true,
      use_metadeps: false,
    }
  }
}

impl ToBExpr for Config {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "use_build_rs" => self.use_build_rs.to_expr(),
      "use_metadeps" => self.use_metadeps.to_expr()
    }
  }
}

#[derive(Debug, Clone)]
pub struct Dependency {
  pub name: String,
  pub version: String,
}

impl ToBExpr for Dependency {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "name" => b_value!(self.name),
      "version" => b_value!(self.version)
    }
  }
}

#[derive(Debug, Clone)]
pub struct Platform {
  triple: String,
  flags: Vec<String>,
  vars: Vec<(String, String)>,
}

impl Platform {
  pub fn new(triple: &str, platform_attrs: &Vec<CargoCfg>) -> Platform {
    Platform {
      triple: triple.to_owned(),
      flags: platform_attrs.iter()
        .filter_map(take_flag_from_attr).collect(),
      vars: platform_attrs.iter()
        .filter_map(take_var_from_attr).collect(),
    }
  }
}

fn take_flag_from_attr(attr: &CargoCfg) -> Option<String> {
  if let &CargoCfg::Name(ref s) = attr {
    Some(s.clone())
  } else {
    None
  }
}

fn take_var_from_attr(attr: &CargoCfg) -> Option<(String, String)> {
  if let &CargoCfg::KeyPair(ref k, ref v) = attr {
    Some((k.clone(), v.clone()))
  } else {
    None
  }
}

impl ToBExpr for Platform {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "triple" => self.triple.to_expr(),
      "flags" => self.flags.to_expr(),
      "vars" => self.vars.to_expr()
    }
  }
}

#[derive(Debug, Clone)]
pub struct Workspace {
  pub platform: Platform,
  pub packages: Vec<Package>,
}

impl Workspace {
  pub fn new(packages: &Vec<Package>,
             platform_triple: &str,
             platform_attrs: &Vec<CargoCfg>) -> Workspace {
    Workspace {
      packages: packages.clone(),
      platform: Platform::new(platform_triple, platform_attrs),
    }
  }
}

impl ToBExpr for Workspace {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "platform" => self.platform.to_expr(),
      "packages" => self.packages.to_expr()
    }
  }
}

#[derive(Debug, Clone)]
pub struct Target {
  pub name: String,
  pub kinds: Vec<String>,
  pub path: String,
}

impl ToBExpr for Target {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "name" => b_value!(self.name),
      "kinds" => self.kinds.to_expr(),
      "path" => b_value!(self.path)
    }
  }
}

#[derive(Debug, Clone)]
pub struct Package {
  pub id: PackageId,
  pub package: CargoPackage,
  pub features: HashSet<String>,
  pub full_name: String,
  pub path: String,
  pub dependencies: Vec<Dependency>,
  pub build_dependencies: Vec<Dependency>,
  pub dev_dependencies: Vec<Dependency>,
  pub is_root_dependency: bool,
  pub targets: Vec<Target>,
  pub bazel_config: Config,
}


impl ToBExpr for Package {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "package" => b_struct! {
        "pkg_name" => b_value!(self.id.name()),
        "pkg_version" => b_value!(self.id.version())
      },
      "bazel_config" => self.bazel_config.to_expr(),
      "dependencies" => self.dependencies.to_expr(),
      "build_dependencies" => self.build_dependencies.to_expr(),
      "dev_dependencies" => self.dev_dependencies.to_expr(),
      "features" => self.features.to_expr(),
      "targets" => self.targets.to_expr()
    }
  }
}
