use files::BExpr;
use files::ToBExpr;
use cargo::util::Cfg as CargoCfg;
use cargo::core::Package as CargoPackage;
use std::collections::HashSet;
use cargo::core::PackageId;
use std::cmp::Ordering;

static OVERRIDE_FILE_VERSION: &'static str = "1";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl PartialOrd for Package {
  fn partial_cmp(&self, other: &Package) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Package {
  fn cmp(&self, other: &Package) -> Ordering {
    let name_cmp = self.id.name().cmp(&other.id.name());

    if name_cmp != Ordering::Equal {
      return name_cmp
    } else {
      return self.id.version().cmp(&other.id.version())
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OverrideSettings {
  internal_override_file_version: String,
  pub global_settings: GlobalOverrideSettings,
}

impl Default for OverrideSettings {
  fn default() -> OverrideSettings {
    OverrideSettings {
      internal_override_file_version: OVERRIDE_FILE_VERSION.to_owned(),
      global_settings: GlobalOverrideSettings::default(),
    }
  }
}

impl ToBExpr for OverrideSettings {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "internal_override_file_version" => self.internal_override_file_version.to_expr(),
      "global_settings" => self.global_settings.to_expr()
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlobalOverrideSettings {
  pub dependency_replacements: Vec<DependencyReplacement>,
}

impl Default for GlobalOverrideSettings {
  fn default() -> GlobalOverrideSettings {
    GlobalOverrideSettings {
      // This default value is an unlikely crate name, as an example for users
      dependency_replacements: vec![DependencyReplacement {
        pkg_name: "foo_bar_baz".to_owned(),
        pkg_version: "8.8.8".to_owned(),
        target: "//foo/bar:baz".to_owned(),
      }],
    }
  }
}

impl ToBExpr for GlobalOverrideSettings {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "dependency_replacements" => self.dependency_replacements.to_expr()
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyReplacement {
  pub pkg_name: String,
  pub pkg_version: String,
  pub target: String,
}

impl ToBExpr for DependencyReplacement {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "pkg_name" => self.pkg_name.to_expr(),
      "pkg_version" => self.pkg_version.to_expr(),
      "target" => self.target.to_expr()
    }
  }
}
