use bazel;

use cargo::CargoError;
use cargo::core::Dependency;
use cargo::core::PackageId;
use cargo::core::Package as CargoPackage;
use cargo::core::PackageSet;
use cargo::core::SourceId;
use cargo::core::Resolve;
use cargo::core::TargetKind;
use cargo::core::Workspace;
use cargo::core::dependency::Kind;
use cargo::ops::Packages;
use cargo::ops;
use cargo::util::Cfg;
use cargo::util::ChainError;
use cargo::util::Config;
use cargo::util::ToUrl;
use cargo::util::human;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::str;

/** The set of all included dependencies for Cargo's dependency categories. */
pub struct PlannedDeps {
    pub build_deps: Vec<bazel::Dependency>,
    pub dev_deps: Vec<bazel::Dependency>,
    pub normal_deps: Vec<bazel::Dependency>,
}

impl PlannedDeps {
    /**
     * Identifies the full set of cargo dependencies for the provided package id using cargo's
     * resolution details.
     */
    pub fn find_all_deps(id: &PackageId, package: &CargoPackage, resolve: &Resolve, platform_triple: &str, platform_attrs: &Vec<Cfg>) -> PlannedDeps {
        let platform_deps = package
            .dependencies()
            .iter()
            .filter(|dep| dep.platform().map(|p| p.matches(&platform_triple, Some(&platform_attrs))).unwrap_or(true))
            .cloned()
            .collect::<Vec<Dependency>>();
        let build_deps = take_kinded_dep_names(&platform_deps, Kind::Build);
        let dev_deps = take_kinded_dep_names(&platform_deps, Kind::Development);
        let normal_deps = take_kinded_dep_names(&platform_deps, Kind::Normal);
        let resolved_deps = resolve.deps(&id).into_iter()
            .map(|dep| bazel::Dependency {
                name: dep.name().to_owned(),
                version: dep.version().to_string(),
            })
            .collect::<Vec<bazel::Dependency>>();

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
    pub fn resolve_from_files(config: &Config) -> Result<ResolvedPlan, Box<CargoError>> {
        let lockfile = Path::new("Cargo.lock");
        let manifest_path = lockfile.parent().unwrap().join("Cargo.toml");
        let manifest = env::current_dir().unwrap().join(&manifest_path);
        let ws = try!(Workspace::new(&manifest, config));
        let specs = Packages::All.into_package_id_specs(&ws).chain_error(|| {
          human("failed to fully parse package definitions")
        })?;
        let root_name = specs.iter().next().unwrap().name().to_owned();

        let (packages, resolve) = ops::resolve_ws_precisely(
                &ws,
                None,
                &[],
                false,
                false,
                &specs).chain_error(|| {
            human("failed to load pkg lockfile")
        })?;

        Ok(ResolvedPlan {
          root_name: root_name,
          packages: packages,
          resolve: resolve,
        })
    }
}

/** Derives bazel target objects from Cargo's target information. */
pub fn identify_targets(full_name: &str, package: &CargoPackage) -> Result<Vec<bazel::Target>, Box<CargoError>> {
    let partial_path = format!("{}/", full_name);
    let partial_path_byte_length = partial_path.as_bytes().len();
    let mut targets = Vec::new();

    for target in package.targets().iter() {
        let target_path_str = try!(target.src_path().to_str()
          .ok_or(human(format!("path for {}'s target {} wasn't unicode", &full_name, target.name()))))
          .to_owned();
        let crate_name_str_idx = try!(target_path_str.find(&partial_path)
          .ok_or(human(format!("path for {}'s target {} should have been in vendor directory", &full_name, target.name()))));
        let local_path_bytes = target_path_str.bytes()
          .skip(crate_name_str_idx + partial_path_byte_length)
          .collect::<Vec<_>>();
        let local_path_str = String::from_utf8(local_path_bytes).unwrap();

        targets.push(bazel::Target {
          name: target.name().to_owned(),
          path: local_path_str,
          kinds: kind_to_kinds(target.kind()),
        });
    }

    Ok(targets)
}

/** Enumerates the set of all possibly relevant packages for the Cargo dependencies */
pub fn find_all_package_ids(flag_host: Option<String>, config: &Config, resolve: &Resolve) -> Result<Vec<PackageId>, Box<CargoError>> {
    let source_id_from_registry =
      flag_host.map(|s| s.to_url().map(|url| SourceId::for_registry(&url)).map_err(human));

    let registry_id = try!(source_id_from_registry.unwrap_or_else(|| SourceId::crates_io(config)));
    try!(fs::metadata("Cargo.lock").chain_error(|| {
      human("failed to find Cargo.lock. Please run `cargo generate-lockfile` first.")
    }));

    let mut package_ids = resolve.iter()
        .filter(|id| *id.source_id() == registry_id)
        .cloned()
        .collect::<Vec<_>>();
    package_ids.sort_by_key(|id| id.name().to_owned());
    Ok(package_ids)
}

/** Extracts the dependencies that are of the provided kind. */
fn take_kinded_dep_names(platform_deps: &Vec<Dependency>, kind: Kind) -> HashSet<String> {
  platform_deps
    .iter()
    .filter(|d| d.kind() == kind)
    .map(|dep| dep.name().to_owned())
    .collect()
}

/**
 * Extracts consistently named Strings for the provided TargetKind.
 *
 * TODO(acmcarther): Remove this shim borrowed from Cargo when Cargo is upgraded
 */
fn kind_to_kinds(kind: &TargetKind) -> Vec<String> {
    match kind {
        &TargetKind::Lib(ref kinds) => kinds.iter().map(|k| k.crate_type().to_owned()).collect(),
        &TargetKind::Bin => vec!["bin".to_owned()],
        &TargetKind::ExampleBin | &TargetKind::ExampleLib(_) => vec!["example".to_owned()],
        &TargetKind::Test => vec!["test".to_owned()],
        &TargetKind::CustomBuild => vec!["custom-build".to_owned()],
        &TargetKind::Bench => vec!["bench".to_owned()],
    }
}

