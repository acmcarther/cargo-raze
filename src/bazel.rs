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

#[cfg(test)]
mod tests {
  pub use super::*;
  pub use planning::PlannedBuild;
  pub use rendering::RenderDetails;
  pub use rendering::FileOutputs;
  pub use context::*;
  pub use hamcrest::prelude::*;
  pub use hamcrest::core::expect;

  fn dummy_render_details() -> RenderDetails {
    RenderDetails {
      path_prefix: "./some_render_prefix".to_owned(),
    }
  }

  fn dummy_planned_build(crate_contexts: Vec<CrateContext>) -> PlannedBuild {
    PlannedBuild {
      workspace_context: WorkspaceContext {
        workspace_prefix: "//workspace/prefix".to_owned(),
        platform_triple: "irrelevant".to_owned(),
      },
      crate_contexts: crate_contexts,
    }
  }

  fn dummy_binary_crate() -> CrateContext {
    CrateContext {
      pkg_name: "test-binary".to_owned(),
      pkg_version: "1.1.1".to_owned(),
      features: vec!["feature1".to_owned(), "feature2".to_owned()].to_owned(),
      path: "vendor/test-binary-1.1.1/".to_owned(),
      dependencies: Vec::new(),
      build_dependencies: Vec::new(),
      dev_dependencies: Vec::new(),
      is_root_dependency: true,
      metadeps: Vec::new(),
      platform_triple: "irrelevant".to_owned(),
      targets: vec![
        BuildTarget {
          name: "some_binary".to_owned(),
          kind: "bin".to_owned(),
          path: "bin/main.rs".to_owned()
        }
      ],
      build_script_target: None,
    }
  }

  fn dummy_library_crate() -> CrateContext {
    CrateContext {
      pkg_name: "test-library".to_owned(),
      pkg_version: "1.1.1".to_owned(),
      features: vec!["feature1".to_owned(), "feature2".to_owned()].to_owned(),
      path: "vendor/test-library-1.1.1/".to_owned(),
      dependencies: Vec::new(),
      build_dependencies: Vec::new(),
      dev_dependencies: Vec::new(),
      is_root_dependency: true,
      metadeps: Vec::new(),
      platform_triple: "irrelevant".to_owned(),
      targets: vec![
        BuildTarget {
          name: "some_library".to_owned(),
          kind: "lib".to_owned(),
          path: "path/lib.rs".to_owned()
        }
      ],
      build_script_target: None,
    }
  }

  fn extract_contents_matching_path(file_outputs: &Vec<FileOutputs>, crate_name: &str) -> String {
    let mut matching_files_contents = file_outputs
      .iter()
      .filter(|output| output.path.contains(crate_name))
      .map(|output| output.contents.to_owned())
      .collect::<Vec<String>>();

    assert_that!(matching_files_contents.len(), equal_to(1));
    matching_files_contents.pop().unwrap()
  }

  fn render_crates_for_test(contexts: Vec<CrateContext>) -> Vec<FileOutputs> {
    BazelRenderer::new().render_planned_build(
      &dummy_render_details(),
      &dummy_planned_build(contexts)) .unwrap()
  }

  #[test]
  fn all_plans_contain_root_build_file() {
    let file_outputs = render_crates_for_test(Vec::new());
    let file_names = file_outputs.iter().map(|output| output.path.as_ref()).collect::<Vec<&str>>();

    assert_that!(&file_names, contains(vec!["./some_render_prefix/vendor/BUILD"]).exactly());
  }

  #[test]
  fn crates_generate_build_files() {
    let file_outputs = render_crates_for_test(vec![dummy_library_crate()]);
    let file_names = file_outputs.iter().map(|output| output.path.as_ref()).collect::<Vec<&str>>();

    assert_that!(&file_names,
                 contains(vec!["./some_render_prefix/vendor/BUILD", "./some_render_prefix/vendor/test-library-1.1.1/BUILD"]).exactly());
  }

  #[test]
  fn root_crates_get_build_aliases() {
    let file_outputs = render_crates_for_test(vec![dummy_library_crate()]);
    let root_build_contents = extract_contents_matching_path(&file_outputs, "vendor/BUILD");

    expect(root_build_contents.contains("alias"),
      format!("expected root build contents to contain an alias \
              for test-library crate, but it just contained [{}]", root_build_contents)).unwrap();
  }

  #[test]
  fn non_root_crates_dont_get_build_aliases() {
    let mut non_root_crate = dummy_library_crate();
    non_root_crate.is_root_dependency = false;

    let file_outputs = render_crates_for_test(vec![non_root_crate]);
    let root_build_contents = extract_contents_matching_path(&file_outputs, "vendor/BUILD");

    expect(!root_build_contents.contains("alias"),
      format!("expected root build contents not to contain an alias \
              for test-library crate, but it just contained [{}]", root_build_contents)).unwrap();
  }

  #[test]
  fn binaries_get_rust_binary_rules() {
    let file_outputs = render_crates_for_test(vec![dummy_binary_crate()]);
    let crate_build_contents = extract_contents_matching_path(&file_outputs, "vendor/test-binary-1.1.1/BUILD");

    expect(crate_build_contents.contains("rust_binary("),
      format!("expected crate build contents to contain rust_binary, \
              but it just contained [{}]", crate_build_contents)).unwrap();
  }

  #[test]
  fn libraries_get_rust_library_rules() {
    let file_outputs = render_crates_for_test(vec![dummy_library_crate()]);
    let crate_build_contents = extract_contents_matching_path(&file_outputs, "vendor/test-library-1.1.1/BUILD");

    expect(crate_build_contents.contains("rust_library("),
      format!("expected crate build contents to contain rust_library, \
              but it just contained [{}]", crate_build_contents)).unwrap();
  }
}
