#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BuildDependency {
  pub name: String,
  pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct BuildTarget {
  pub name: String,
  pub kind: String,
  pub path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrateContext {
  pub pkg_name: String,
  pub pkg_version: String,
  pub features: Vec<String>,
  pub dependencies: Vec<BuildDependency>,
  pub build_dependencies: Vec<BuildDependency>,
  pub dev_dependencies: Vec<BuildDependency>,
  pub is_root_dependency: bool,
  pub targets: Vec<BuildTarget>,
  pub build_script_target: Option<BuildTarget>,
  // TODO(acmcarther): Consider plugin topic
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceContext {
  pub crates: Vec<CrateContext>,
}
