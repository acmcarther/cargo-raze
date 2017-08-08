use cargo::CargoError;
use cargo::core::Dependency;
use cargo::core::Package as CargoPackage;
use cargo::core::PackageId;
use cargo::core::PackageSet;
use cargo::core::Resolve;
use cargo::core::SourceId;
use cargo::core::Workspace;
use cargo::core::dependency::Kind;
use cargo::ops::Packages;
use cargo::ops;
use cargo::util::CargoResult;
use cargo::util::Cfg;
use cargo::util::Config;
use cargo::util::ToUrl;
use context::BazelConfig;
use context::BazelDependency;
use context::BazelTarget;
use context::CrateContext;
use context::WorkspaceContext;
use rendering::Renderer;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::str;
use util;

pub struct PlannedBuild {
  workspace_context: WorkspaceContext,
  crate_contexts: Vec<CrateContext>,
}

impl PlannedBuild {
  pub fn new(workspace_context: WorkspaceContext, crate_contexts: Vec<CrateContext>) -> PlannedBuild {
    PlannedBuild {
      workspace_context: workspace_context,
      crate_contexts: crate_contexts,
    }
  }

  pub fn render(&self) -> CargoResult<Vec<FileOutputs>> {
    let renderer = Renderer::new(self.workspace_context.clone());

    let mut file_outputs = Vec::new();
    for package in &self.crate_contexts {
      let build_file_path = format!("{}BUILD", &package.path);
      let rendered_crate_build_file = try!(renderer.render_crate(&package).map_err(|e| CargoError::from(e.to_string())));
      file_outputs.push(FileOutputs { path: build_file_path, contents: rendered_crate_build_file })
    }

    let build_file_path = "vendor/BUILD".to_owned();
    let rendered_alias_build_file = try!(renderer.render_aliases(&self.crate_contexts).map_err(|e| CargoError::from(e.to_string())));
    file_outputs.push(FileOutputs { path: build_file_path, contents: rendered_alias_build_file });
    Ok(file_outputs)
  }
}

pub struct FileOutputs {
  pub path: String,
  pub contents: String
}

pub struct BuildPlanner<'a> {
  workspace_prefix: String,
  platform_triple: String,
  config: &'a Config,
  platform_attrs: Vec<Cfg>,
  registry: Option<SourceId>,
}

impl <'a>  BuildPlanner<'a> {
  pub fn new(workspace_prefix: String, platform_triple: String, config: &'a Config) -> CargoResult<BuildPlanner<'a>> {
    Ok(BuildPlanner {
      workspace_prefix: workspace_prefix,
      platform_attrs: try!(util::fetch_attrs(&platform_triple)),
      platform_triple: platform_triple,
      config: config,
      registry: None,
    })
  }

  pub fn set_registry_from_url(&mut self, host: String) -> CargoResult<()> {
    match host.to_url().map(|url| SourceId::for_registry(&url)) {
      Ok(registry_id) => {
        self.registry = Some(registry_id);
        Ok(())
      },
      Err(value) => Err(CargoError::from(value))
    }
  }

  pub fn plan_build(&self) -> CargoResult<PlannedBuild> {
      let ResolvedPlan {root_name, packages, resolve} =
          try!(ResolvedPlan::resolve_from_files(&self.config));

      let root_package_id = try!(resolve.iter()
          .filter(|dep| dep.name() == root_name)
          .next()
          .ok_or(CargoError::from("root crate should be in cargo resolve")));
      let root_direct_deps = resolve.deps(&root_package_id).cloned().collect::<HashSet<_>>();

      let mut crate_contexts = Vec::new();

      let source_id = match self.registry.clone() {
        Some(v) => v,
        None => try!(SourceId::crates_io(&self.config)),
      };

      for id in try!(find_all_package_ids(source_id, &resolve)) {
          let package = packages.get(&id).unwrap().clone();
          let mut features = resolve.features(&id).clone().into_iter().collect::<Vec<_>>();
          features.sort();
          let full_name = format!("{}-{}", id.name(), id.version());
          let path = format!("./vendor/{}-{}/", id.name(), id.version());

          // Verify that package is really vendored
          try!(fs::metadata(&path).map_err(|_| {
              CargoError::from(format!("failed to find {}. Please run `cargo vendor -x` first.", &path))
          }));

          // Identify all possible dependencies
          let PlannedDeps { mut build_deps, mut dev_deps, mut normal_deps } =
              PlannedDeps::find_all_deps(&id, &package, &resolve, &self.platform_triple, &self.platform_attrs);
          build_deps.sort();
          dev_deps.sort();
          normal_deps.sort();

          let mut targets = try!(identify_targets(&full_name, &package));
          targets.sort();
          let build_script_target = targets.iter().find(|t| t.kind.deref() == "custom-build").cloned();
          let targets_sans_build_script =
            targets.into_iter().filter(|t| t.kind.deref() != "custom-build").collect::<Vec<_>>();

          crate_contexts.push(CrateContext {
              pkg_name: id.name().to_owned(),
              pkg_version: id.version().to_string(),
              features: features,
              is_root_dependency: root_direct_deps.contains(&id),
              metadeps: Vec::new() /* TODO(acmcarther) */,
              dependencies: normal_deps,
              build_dependencies: build_deps,
              dev_dependencies: dev_deps,
              path: path,
              build_script_target: build_script_target,
              targets: targets_sans_build_script,
              bazel_config: BazelConfig::default(),
              platform_triple: self.platform_triple.to_owned(),
          });
      }

      let workspace_context = WorkspaceContext {
        workspace_prefix: self.workspace_prefix.clone(),
        platform_triple: self.platform_triple.clone(),
      };
      Ok(PlannedBuild::new(workspace_context, crate_contexts))
  }
}

/** The set of all included dependencies for Cargo's dependency categories. */
pub struct PlannedDeps {
    pub build_deps: Vec<BazelDependency>,
    pub dev_deps: Vec<BazelDependency>,
    pub normal_deps: Vec<BazelDependency>,
}

impl PlannedDeps {
    /**
     * Identifies the full set of cargo dependencies for the provided package id using cargo's
     * resolution details.
     */
    pub fn find_all_deps(id: &PackageId,
                         package: &CargoPackage,
                         resolve: &Resolve,
                         platform_triple: &str,
                         platform_attrs: &Vec<Cfg>) -> PlannedDeps {
        let platform_deps = package
            .dependencies()
            .iter()
            .filter(|dep| {
                dep.platform()
                    .map(|p| p.matches(&platform_triple, Some(&platform_attrs)))
                    .unwrap_or(true)
            })
            .cloned()
            .collect::<Vec<Dependency>>();
        let build_deps = util::take_kinded_dep_names(&platform_deps, Kind::Build);
        let dev_deps = util::take_kinded_dep_names(&platform_deps, Kind::Development);
        let normal_deps = util::take_kinded_dep_names(&platform_deps, Kind::Normal);
        let resolved_deps = resolve.deps(&id).into_iter()
            .map(|dep| BazelDependency {
                name: dep.name().to_owned(),
                version: dep.version().to_string(),
            })
            .collect::<Vec<BazelDependency>>();

        PlannedDeps {
           normal_deps:
               resolved_deps.iter().filter(|d| normal_deps.contains(&d.name)).cloned().collect(),
           build_deps:
               resolved_deps.iter().filter(|d| build_deps.contains(&d.name)).cloned().collect(),
           dev_deps:
               resolved_deps.into_iter().filter(|d| dev_deps.contains(&d.name)).collect(),
        }
    }
}

/** A synthesized Cargo dependency resolution. */
pub struct ResolvedPlan<'a> {
    pub root_name: String,
    pub packages: PackageSet<'a>,
    pub resolve: Resolve,
}

impl<'a> ResolvedPlan<'a> {
    /**
     * Performs Cargo's own build plan resolution, yielding the root crate, the set of packages, and
     * the resolution graph.
     */
    pub fn resolve_from_files(config: &Config) -> CargoResult<ResolvedPlan> {
        let lockfile = Path::new("Cargo.lock");
        let manifest_path = lockfile.parent().unwrap().join("Cargo.toml");
        let manifest = env::current_dir().unwrap().join(&manifest_path);
        let ws = try!(Workspace::new(&manifest, config));
        let specs = Packages::All.into_package_id_specs(&ws)?;
        let root_name = specs.iter().next().unwrap().name().to_owned();

        let (packages, resolve) = ops::resolve_ws_precisely(
                &ws,
                None,
                &[],
                false,
                false,
                &specs)?;

        Ok(ResolvedPlan {
          root_name: root_name,
          packages: packages,
          resolve: resolve,
        })
    }
}

/** Enumerates the set of all possibly relevant packages for the Cargo dependencies */
fn find_all_package_ids(registry_id: SourceId, resolve: &Resolve) -> CargoResult<Vec<PackageId>> {
    try!(fs::metadata("Cargo.lock").map_err(|_| {
      CargoError::from("failed to find Cargo.lock. Please run `cargo generate-lockfile` first.")
    }));

    let mut package_ids = resolve.iter()
        .filter(|id| *id.source_id() == registry_id)
        .cloned()
        .collect::<Vec<_>>();
    package_ids.sort_by_key(|id| id.name().to_owned());
    Ok(package_ids)
}


/** Derives bazel target objects from Cargo's target information. */
fn identify_targets(full_name: &str, package: &CargoPackage) -> CargoResult<Vec<BazelTarget>> {
    let partial_path = format!("{}/", full_name);
    let partial_path_byte_length = partial_path.as_bytes().len();
    let mut targets = Vec::new();

    for target in package.targets().iter() {
        let target_path_str = try!(target.src_path().to_str()
          .ok_or(CargoError::from(format!("path for {}'s target {} wasn't unicode", &full_name, target.name()))))
          .to_owned();
        let crate_name_str_idx = try!(target_path_str.find(&partial_path)
          .ok_or(CargoError::from(format!("path for {}'s target {} should have been in vendor directory", &full_name, target.name()))));
        let local_path_bytes = target_path_str.bytes()
          .skip(crate_name_str_idx + partial_path_byte_length)
          .collect::<Vec<_>>();
        let local_path_str = String::from_utf8(local_path_bytes).unwrap();
        for kind in util::kind_to_kinds(target.kind()) {
          targets.push(BazelTarget {
            name: target.name().to_owned(),
            path: local_path_str.clone(),
            kind: kind,
          });
        }
    }

    Ok(targets)
}
