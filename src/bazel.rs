use cargo::util::CargoResult;
use cargo::CargoError;
use context::CrateContext;
use context::WorkspaceContext;
use planning::PlannedBuild;
use rendering::BuildRenderer;
use rendering::FileOutputs;
use rendering::RenderDetails;
use tera::Context;
use tera::Tera;
use tera;

pub struct BazelRenderer {
  internal_renderer: Tera,
}

impl BazelRenderer {
  pub fn new() -> BazelRenderer {
    // Configure tera with a bogus template dir: We don't want any runtime template support
    let mut renderer = Tera::new("src/not/a/dir/*").unwrap();
    renderer.add_raw_templates(vec![
      ("templates/partials/rust_binary.template", include_str!("templates/partials/rust_binary.template")),
      ("templates/partials/rust_library.template", include_str!("templates/partials/rust_library.template")),
      ("templates/partials/rust_test.template", include_str!("templates/partials/rust_test.template")),
      ("templates/partials/rust_bench_test.template", include_str!("templates/partials/rust_bench_test.template")),
      ("templates/partials/rust_example.template", include_str!("templates/partials/rust_example.template")),
      ("templates/partials/build_script.template", include_str!("templates/partials/build_script.template")),
      ("templates/workspace.BUILD.template", include_str!("templates/workspace.BUILD.template")),
      ("templates/crate.BUILD.template", include_str!("templates/crate.BUILD.template"))]).unwrap();

    BazelRenderer {
      internal_renderer: renderer,
    }
  }

  pub fn render_crate(&self, workspace_context: &WorkspaceContext, package: &CrateContext) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.add("path_prefix", &workspace_context.workspace_prefix);
    context.add("crate", &package);
    self.internal_renderer.render("templates/crate.BUILD.template", &context)
  }

  pub fn render_aliases(&self, workspace_context: &WorkspaceContext, all_packages: &Vec<CrateContext>) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.add("path_prefix", &workspace_context.workspace_prefix);
    context.add("crates", &all_packages);
    self.internal_renderer.render("templates/workspace.BUILD.template", &context)
  }
}

impl BuildRenderer for BazelRenderer {
  fn render_planned_build(&mut self, render_details: &RenderDetails, planned_build: &PlannedBuild) -> CargoResult<Vec<FileOutputs>> {
    let &RenderDetails { ref path_prefix, .. } = render_details;
    let &PlannedBuild { ref workspace_context, ref crate_contexts, .. } = planned_build;
    let mut file_outputs = Vec::new();

    for package in crate_contexts {
      let build_file_path = format!("{}/{}BUILD", &path_prefix, &package.path);
      let rendered_crate_build_file = try!(self.render_crate(&workspace_context, &package).map_err(|e| CargoError::from(e.to_string())));
      file_outputs.push(FileOutputs { path: build_file_path, contents: rendered_crate_build_file })
    }

    let build_file_path = format!("{}/vendor/BUILD", &path_prefix);
    let rendered_alias_build_file = try!(self.render_aliases(&workspace_context, &crate_contexts).map_err(|e| CargoError::from(e.to_string())));
    file_outputs.push(FileOutputs { path: build_file_path, contents: rendered_alias_build_file });
    Ok(file_outputs)
  }
}
