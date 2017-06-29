use files::BExpr;
use files::ToBExpr;
use cargo::util::Cfg as CargoCfg;
use cargo::core::Package as CargoPackage;
use std::collections::HashSet;
use cargo::core::PackageId;

static OVERRIDE_FILE_VERSION: &'static str = "1";

/** An object that can provide a useful example value, similar to Default */
pub trait ExampleValue {
  fn example_value() -> Self;
}

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
  pub packages: Vec<CrateConfig>,
}

impl Workspace {
  pub fn new(packages: &Vec<Package>,
             platform_triple: &str,
             platform_attrs: &Vec<CargoCfg>) -> Workspace {
    Workspace {
      packages: packages.iter().map(|p| p.to_crate_config()).collect(),
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Metadep {
  pub name: String,
  pub min_version: String,
}

impl ToBExpr for Metadep {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "name" => self.name.to_expr(),
      "min_version" => self.min_version.to_expr()
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
  pub metadeps: Vec<Metadep>,
}

impl Package {
  pub fn to_crate_config(&self) -> CrateConfig {
    CrateConfig {
      package: PackageIdent {
        pkg_name: self.id.name().to_owned(),
        pkg_version: self.id.version().to_string(),
      },
      bazel_config: self.bazel_config.clone(),
      metadeps: self.metadeps.clone(),
      dependencies: self.dependencies.clone(),
      build_dependencies: self.build_dependencies.clone(),
      dev_dependencies: self.dev_dependencies.clone(),
      features: self.features.iter().cloned().collect(),
      targets: self.targets.clone(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PackageIdent {
  pub pkg_name: String,
  pub pkg_version: String,
}

impl ToBExpr for PackageIdent {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "pkg_name" => self.pkg_name.to_expr(),
      "pkg_version" => self.pkg_version.to_expr()
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrateConfig {
  pub package: PackageIdent,
  pub bazel_config: Config,
  pub metadeps: Vec<Metadep>,
  pub dependencies: Vec<Dependency>,
  pub build_dependencies: Vec<Dependency>,
  pub dev_dependencies: Vec<Dependency>,
  pub features: Vec<String>,
  pub targets: Vec<Target>,
}

impl ToBExpr for CrateConfig {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "package" => self.package.to_expr(),
      "bazel_config" => self.bazel_config.to_expr(),
      "metadeps" => self.metadeps.to_expr(),
      "dependencies" => self.dependencies.to_expr(),
      "build_dependencies" => self.build_dependencies.to_expr(),
      "dev_dependencies" => self.dev_dependencies.to_expr(),
      "features" => self.features.to_expr(),
      "targets" => self.targets.to_expr()
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OverrideSettings {
  internal_override_file_version: String,
  pub dependency_overrides: Vec<DependencyOverride>,
}

impl ExampleValue for OverrideSettings {
  fn example_value() -> OverrideSettings {
    OverrideSettings {
      internal_override_file_version: OVERRIDE_FILE_VERSION.to_owned(),
      // This default value is an unlikely crate name, as an example for users
      dependency_overrides: vec![
        DependencyOverride {
          pkg_name: "foo_bar_baz".to_owned(),
          pkg_version: "8.8.8".to_owned(),
          target_replacement: Some("//foo/bar:baz".to_owned()),
          config_replacement: None
        }, DependencyOverride {
          pkg_name: "foo_bar_qux".to_owned(),
          pkg_version: "8.8.8".to_owned(),
          target_replacement: None,
          config_replacement: Some(CrateConfig {
            package: PackageIdent {
              pkg_name: "foo_bar_qux".to_owned(),
              pkg_version: "8.8.88".to_owned(),
            },
            bazel_config: Config {
              use_build_rs: false,
              use_metadeps: false,
            },
            metadeps: Vec::new(),
            dependencies: Vec::new(),
            build_dependencies: Vec::new(),
            dev_dependencies: Vec::new(),
            features: Vec::new(),
            targets: Vec::new(),
          })
        }
      ],
    }
  }
}

impl ToBExpr for OverrideSettings {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "internal_override_file_version" => self.internal_override_file_version.to_expr(),
      "dependency_overrides" => self.dependency_overrides.to_expr()
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyOverride {
  pub pkg_name: String,
  pub pkg_version: String,
  pub target_replacement: Option<String>,
  pub config_replacement: Option<CrateConfig>,
}

impl ToBExpr for DependencyOverride {
  fn to_expr(&self) -> BExpr {
    b_struct! {
      "pkg_name" => self.pkg_name.to_expr(),
      "pkg_version" => self.pkg_version.to_expr(),
      "target_replacement" => self.target_replacement.to_expr(),
      "config_replacement" => self.config_replacement.to_expr()
    }
  }
}
