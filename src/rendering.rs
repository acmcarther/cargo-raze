use context::CrateContext;
use context::WorkspaceContext;
use tera::Context;
use tera::Tera;
use tera;

pub struct Renderer {
  renderer: Tera,
  workspace_context: WorkspaceContext,
}

impl Renderer {
  pub fn new(workspace_context: WorkspaceContext) -> Renderer {
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

    Renderer {
      renderer: renderer,
      workspace_context: workspace_context,
    }
  }

  pub fn render_crate(&self, package: &CrateContext) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.add("path_prefix", &self.workspace_context.workspace_prefix);
    context.add("crate", &package);
    self.renderer.render("templates/crate.BUILD.template", &context)
  }

  pub fn render_aliases(&self, all_packages: &Vec<CrateContext>) -> Result<String, tera::Error> {
    let mut context = Context::new();
    context.add("path_prefix", &self.workspace_context.workspace_prefix);
    context.add("crates", &all_packages);
    self.renderer.render("templates/workspace.BUILD.template", &context)
  }
}

