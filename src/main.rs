extern crate cargo as __internal_cargo_do_not_use;
#[macro_use(Serialize, Deserialize)]
extern crate serde_derive;
extern crate serde;
extern crate itertools;
extern crate tera;
#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;
extern crate toml;
extern crate url;

#[cfg(test)]
#[macro_use]
extern crate hamcrest;

mod util;
mod init;
mod context;

mod _cargo {
  pub use __internal_cargo_do_not_use::CargoError as Error;
  pub use __internal_cargo_do_not_use::CliResult;
  pub use __internal_cargo_do_not_use::call_main_without_stdin;
  pub use __internal_cargo_do_not_use::core::Package;
  pub use __internal_cargo_do_not_use::core::PackageIdSpec;
  pub use __internal_cargo_do_not_use::core::Workspace;
  pub use __internal_cargo_do_not_use::core::Summary;
  pub use __internal_cargo_do_not_use::core::Dependency;
  pub use __internal_cargo_do_not_use::core::dependency::Kind;
  pub use __internal_cargo_do_not_use::core::registry::PackageRegistry;
  pub use __internal_cargo_do_not_use::core::registry::Registry;
  pub use __internal_cargo_do_not_use::exit_with_error;
  pub use __internal_cargo_do_not_use::core::resolver::Method;
  pub use __internal_cargo_do_not_use::core::resolver;
  pub use __internal_cargo_do_not_use::core::PackageSet;
  pub use __internal_cargo_do_not_use::core::Resolve;
  pub use __internal_cargo_do_not_use::ops::resolve_with_previous;
  pub use __internal_cargo_do_not_use::ops::resolve_ws_precisely;
  pub use __internal_cargo_do_not_use::ops::Packages;
  pub use __internal_cargo_do_not_use::util::CargoResult as CResult;
  pub use __internal_cargo_do_not_use::util::Config;
  pub use __internal_cargo_do_not_use::util::Cfg;
}

use std::env;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use util::OkButLog;
use context::WorkspaceContext;
use context::CrateContext;
use context::BuildTarget;
use context::BuildDependency;

#[derive(Debug, Deserialize)]
struct Options {
    flag_verbose: u32,
    flag_host: Option<String>,
    flag_color: Option<String>,
    flag_target: Option<String>,
    flag_dryrun: Option<bool>,
    #[serde(rename = "flag_Z")]
    flag_z: Vec<String>,
}

const USAGE: &'static str = r#"
Generate BUILD files for your pre-vendored Cargo dependencies.

Usage:
    cargo raze [options]

Options:
    -h, --help                Print this message
    -v, --verbose             Use verbose output
    --host HOST               Registry index to sync with
    --color WHEN              Coloring: auto, always, never
    --target TARGET           Platform to generate BUILD files for
    -d, --dryrun              Do not emit any files
    -Z FLAG ...               Unstable (nightly-only) flags to Cargo
"#;

fn main() {
  init::global_logger();
  let config = _cargo::Config::default().unwrap();
  let args = env::args().collect::<Vec<_>>();
  let result = _cargo::call_main_without_stdin(cargo_main, &config, USAGE, &args, false);

  if let Err(e) = result {
      _cargo::exit_with_error(e, &mut *config.shell());
  }
}

fn cargo_main(options: Options, config: &_cargo::Config) -> _cargo::CliResult {
  try!(config.configure(options.flag_verbose,
                        Some(false) /* quiet */,
                        &options.flag_color,
                        /* frozen = */ false,
                        /* locked = */ false,
                        &options.flag_z));

  let target = options.flag_target.unwrap();

  let current_dir = env::current_dir().unwrap();

  let workspace = Workspace::load_from_fs(config, current_dir.join(&Path::new("Cargo.toml"))).unwrap();
  let r = CargoDependencyResolver::for_workspace(&workspace).unwrap();
  let attrs = try!(util::fetch_attrs(&target));
  r.resolve(&target, &attrs);

  //debug!("{:#?}", ws);

  Ok(())
}

trait DependencyResolver {
  fn resolve(&self, platform_triple: &str, platform_attrs: &Vec<_cargo::Cfg>) -> _cargo::CResult<WorkspaceContext>;
}

struct CargoDependencyResolver<'cfg> {
  cargo_workspace: _cargo::Workspace<'cfg>
}

impl <'cfg> CargoDependencyResolver<'cfg> {
  pub fn for_workspace(workspace: &Workspace<'cfg>) -> _cargo::CResult<CargoDependencyResolver<'cfg>> {
    let cargo_workspace = try!(_cargo::Workspace::ephemeral(
      workspace.cargo_package.clone(),
      workspace.cargo_config,
      None /* target_dir */,
      true /* require_optional_deps */,
    ));

    Ok(CargoDependencyResolver {
      cargo_workspace: cargo_workspace
    })
  }

  pub fn resolve_using_network(&self,
                               platform_triple: &str,
                               platform_attrs: &Vec<_cargo::Cfg>) -> _cargo::CResult<WorkspaceContext> {
    let specs = try!(_cargo::Packages::All.into_package_id_specs(&self.cargo_workspace));

    // TODO(acmcarther): Figure out how to do this without network
    let (packages, resolve) = try!(_cargo::resolve_ws_precisely(
      &self.cargo_workspace,
      None,
      &[],
      false,
      false,
      &specs));

    let root_crate_name = specs.iter()
      .next()
      .unwrap()
      .name()
      .to_owned();

    self.resolve_directly(
      platform_triple,
      platform_attrs,
      &root_crate_name,
      packages,
      resolve)
  }

  pub fn resolve_directly(&self,
                          platform_triple: &str,
                          platform_attrs: &Vec<_cargo::Cfg>,
                          root_crate_name: &str,
                          packages: _cargo::PackageSet<'cfg>,
                          resolve: _cargo::Resolve) -> _cargo::CResult<WorkspaceContext> {
    info!("resolve: {:#?}", resolve);
    let mut package_ids = resolve.iter().cloned()
      .filter(|dep| dep.name() != root_name)
      .collect::<Vec<_>>();
    package_ids.sort_by_key(|id| id.name());

    let root_package_id = try!(resolve.iter()
        .filter(|dep| dep.name() == root_name)
        .next()
        .ok_or(CargoError::from("root crate should be in cargo resolve")));
    let root_direct_deps = resolve.deps(&root_package_id).cloned().collect::<HashSet<_>>();

    let mut crates = Vec::new();
    for id in package_ids {
      let package = packages.get(&id).unwrap();
      let dependencies = package
        .dependencies()
        .iter()
        .filter(|dep| {
          dep.platform()
              .map(|p| p.matches(&platform_triple, Some(&platform_attrs)))
              .unwrap_or(true)
        })
        .collect::<Vec<_>>();

      let mut build_deps = Vec::new();
      let mut normal_deps = Vec::new();
      for dep in dependencies.into_iter() {
        match dep.kind() {
          _cargo::Kind::Normal => normal_deps.push(BuildDependency {
            name: dep.name(),
            version: dep.version(),
          }),
          _cargo::Kind::Build => build_deps.push(BuildDependency {
            name: dep.name(),
            version: dep.version().to_string(),
          }),
          _ => () /* can't resolve + vendor dev deps, in general */,
        }
      }

      let mut build_script_target = None;
      let mut targets = Vec::new();
      for cargo_target in package.targets() {
        let kinds = match cargo_target.kind() {
            &TargetKind::Lib(ref kinds) => kinds.iter().map(|k| k.crate_type().to_owned()).collect(),
            &TargetKind::Bin => vec!["bin".to_owned()],
            &TargetKind::CustomBuild => {
              build_script_target = Some(BuildTarget {
                name: cargo_target.name().to_owned(),
                kind: "custom-build".to_owned(),
                path: cargo_target.src_path(),
              });
              continue
            }
            _ => continue /* examples, tests, and benches need dev deps */,
        };

        for kind in kinds {
          targets.push(BuildTarget {
            name: cargo_target.name().to_owned(),
            kind: kind.to_owned(),
            path: cargo_target.src_path(),
          })
        }
      }

      crates.push(CrateContext {
        pkg_name: id.name().to_owned(),
        pkg_version: id.version().to_owned().to_string(),
        features: resolve.features(&id).into_iter().cloned().collect::<Vec<_>>(),
        dependencies: normal_deps,
        build_dependencies: build_deps,
        dev_dependencies: Vec::new(),
        is_root_dependency: root_direct_deps.contains(&id),
        targets: targets,
        build_script_target: build_script_target
      });
    }

    Ok(WorkspaceContext {
      crates: Vec::new()
    })
  }
}

impl <'cfg> DependencyResolver for CargoDependencyResolver<'cfg> {
  fn resolve(&self, platform_triple: &str, platform_attrs: &Vec<_cargo::Cfg>) -> _cargo::CResult<WorkspaceContext> {
    self.resolve_using_network(platform_triple, platform_attrs)
  }
}


struct Workspace<'cfg> {
  pub manifest_path: PathBuf,
  pub raw_manifest: toml::Value,
  pub raw_lock: toml::Value,
  pub cargo_package: _cargo::Package,
  pub cargo_config: &'cfg _cargo::Config,
}

impl <'cfg> Workspace<'cfg> {
  pub fn load_from_fs(cargo_config: &'cfg _cargo::Config, manifest_path: PathBuf) -> _cargo::CResult<Workspace<'cfg>> {
    let cargo_package = try!(_cargo::Package::for_path(&manifest_path, &cargo_config));

    let manifest_contents =
      try!(util::load_file_forcefully(&manifest_path, "Cargo.toml in current working directory"));
    let lock_path = manifest_path.parent()
      .unwrap_or(Path::new("./"))
      .join("Cargo.lock");
    let lock_contents =
      try!(util::load_file_forcefully(&lock_path, "Cargo.lock in same dir as toml"));

    let manifest_toml = try!(manifest_contents.parse::<toml::Value>());
    let lock_toml = try!(manifest_contents.parse::<toml::Value>());

    //debug!("{:#?}", specs);

    Ok(Workspace {
      manifest_path: manifest_path,
      raw_manifest: manifest_toml,
      raw_lock: lock_toml,
      cargo_package: cargo_package,
      cargo_config: cargo_config,
    })
  }
}

