use std::collections::HashMap;

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
  pub crates: HashMap<String, HashMap<String, CrateSettings>>
}

fn default_target() -> String {
  "x86_64-unknown-linux-gnu".to_owned()
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrateSettings {
  /** Dependencies to be added to a crate, in the form "//etc".*/
  #[serde(default)]
  pub additional_deps: Vec<String>,

  /** Dependencies to be removed from a crate, in the form "//etc".*/
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
