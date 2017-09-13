use cargo::util::CargoResult;
use planning::PlannedBuild;

pub struct FileOutputs {
  pub path: String,
  pub contents: String
}

pub struct RenderDetails {
  pub path_prefix: String
}

pub trait BuildRenderer {
  fn render_planned_build(&mut self, render_details: &RenderDetails, planned_build: &PlannedBuild) -> CargoResult<Vec<FileOutputs>>;
}
