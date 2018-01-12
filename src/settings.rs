use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum GenMode {
  /** Generate Vendored-style dependencies.
   *
   * This mode assumes that files are vendored (into vendor/), and generates BUILD files
   * accordingly
   */
  Vendored,

  /**
   * Generate Remote-style dependencies.
   *
   * This mode assumes that files are not locally vendored, and generates a workspace-level
   * function that can bring them in.
   */
  Remote,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CargoToml {
  /** The raze settings (the only part of the Cargo.toml we care about. */
  pub raze: RazeSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RazeSettings {
  /** The path to the Cargo.toml working directory. */
  pub workspace_path: String,

  /** The target to generate BUILD rules for. */
  #[serde(default = "default_target")]
  pub target: String,

  /** Any crate-specific configuration. */
  #[serde(default)]
  pub crates: HashMap<String, HashMap<String, CrateSettings>>,

  /**
   * Prefix for generated Bazel workspaces (from workspace_rules)
   *
   * This is only useful with remote genmode.
   *
   * TODO(acmcarther): Does this have a non-bazel analogue?
   */
  #[serde(default = "default_gen_workspace_prefix")]
  pub gen_workspace_prefix: String,

  /** How to generate the dependencies. See GenMode for details. */
  #[serde(default = "default_genmode")]
  pub genmode: GenMode,
}

fn default_target() -> String {
  "x86_64-unknown-linux-gnu".to_owned()
}

fn default_gen_workspace_prefix() -> String {
  "raze".to_owned()
}

fn default_genmode() -> GenMode {
  GenMode::Vendored
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrateSettings {
  /**
   * Dependencies to be added to a crate.
   *
   * Importantly, the format of dependency references depends on the gen mode.
   * Remote: @{gen_workspace_prefix}__{dep_name}__{dep_version_sanitized}/:{dep_name}
   * Vendored: //{workspace_path}/vendor/{dep_name}-{dep_version}:{dep_name}
   *
   * In addition, the added deps must be accessible from a remote workspace under Remote GenMode.
   * Usually, this means they _also_ need to be remote, but a "local" build path prefixed with
   * "@", in the form "@//something_local" may work.
   */
  #[serde(default)]
  pub additional_deps: Vec<String>,

  /**
   * Dependencies to be removed from a crate, in the form "//etc".
   *
   * Importantly, the format of dependency references depends on the gen mode.
   * Remote: @{gen_workspace_prefix}__{dep_name}__{dep_version_sanitized}/:{dep_name}
   * Vendored: //{workspace_path}/vendor/{dep_name}-{dep_version}:{dep_name}
   */
  #[serde(default)]
  pub skipped_deps: Vec<String>,

  /**
  * Library targets that should be aliased in the root BUILD file.
  *
  * This is useful to facilitate using binary utility crates, such as bindgen, as part of genrules.
  */
  #[serde(default)]
  pub extra_aliased_targets: Vec<String>,

  /** Flags to be added to the crate compilation process, in the form "--flag". */
  #[serde(default)]
  pub additional_flags: Vec<String>,

  /**
   * Whether or not to generate the build script that goes with this crate.
   *
   * Many build scripts will not function, as they will still be built hermetically. However, build
   * scripts that merely generate files into OUT_DIR may be functional.
   */
  #[serde(default = "default_gen_buildrs")]
  pub gen_buildrs: bool,

}

fn default_gen_buildrs() -> bool {
  false
}
