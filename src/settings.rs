use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct CargoToml {
  pub raze: RazeSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RazeSettings {
  #[serde(default = "default_vendor_path")]
  pub vendor_path: String,
  #[serde(default = "default_target")]
  pub target: String,
  #[serde(default)]
  pub crates: HashMap<String, HashMap<String, CrateSettings>>
}

fn default_vendor_path() -> String {
  "//".to_owned()
}

fn default_target() -> String {
  "x86_64-unknown-linux-gnu".to_owned()
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrateSettings {
  #[serde(default)]
  additional_deps: Vec<String>,
  #[serde(default)]
  additional_flags: Vec<String>,
}
