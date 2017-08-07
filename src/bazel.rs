#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Dependency {
  pub name: String,
  pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Target {
  pub name: String,
  pub kind: String,
  pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Metadep {
  pub name: String,
  pub min_version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrateContext {
  pub pkg_name: String,
  pub pkg_version: String,
  pub features: Vec<String>,
  pub path: String,
  pub dependencies: Vec<Dependency>,
  pub build_dependencies: Vec<Dependency>,
  pub dev_dependencies: Vec<Dependency>,
  pub is_root_dependency: bool,
  pub bazel_config: Config,
  pub metadeps: Vec<Metadep>,
  pub platform_triple: String,
  pub targets: Vec<Target>,
  pub build_script_target: Option<Target>,
  // TODO(acmcarther): Consider plugin topic
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct WorkspaceContext {
  pub workspace_prefix: String,
  pub platforms: Vec<String>,
}
