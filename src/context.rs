#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BazelConfig {
  pub use_build_rs: bool,
  pub use_metadeps: bool
}

impl Default for BazelConfig {
  fn default() -> BazelConfig {
    BazelConfig {
      use_build_rs: true,
      use_metadeps: false,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BazelDependency {
  pub name: String,
  pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BazelTarget {
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
  pub dependencies: Vec<BazelDependency>,
  pub build_dependencies: Vec<BazelDependency>,
  pub dev_dependencies: Vec<BazelDependency>,
  pub is_root_dependency: bool,
  pub bazel_config: BazelConfig,
  pub metadeps: Vec<Metadep>,
  pub platform_triple: String,
  pub targets: Vec<BazelTarget>,
  pub build_script_target: Option<BazelTarget>,
  // TODO(acmcarther): Consider plugin topic
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct WorkspaceContext {
  pub workspace_prefix: String,
  pub platform_triple: String,
}
