load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_binary")

def _extract_dependency_paths(dependencies, workspace_path):
    """
    Generates the bazel paths to other cargo crates.

    Returns a list of string bazel paths
    """

    deps = []
    for dependency in dependencies:
        dependency_name_sanitized = dependency.name.replace('-', '_')
        deps.append(workspace_path + "vendor/" + dependency.name + '-' + dependency.version + ":" + dependency_name_sanitized)
    return deps

def _resolve_overrides(crate_bzl, cargo_override_bzl):
    """
    Parses the global override config looking for overrides for this crate.

    Returns a struct containing an exact override, if present, and close overrides,
    which differ only by a version
    """

    package = crate_bzl.package
    exact_override = None
    close_overrides = []
    for override in cargo_override_bzl.dependency_overrides:
      if package.pkg_name != override.pkg_name:
        continue
      if package.pkg_version == override.pkg_version:
        if exact_override:
          fail("Package was already set once!")
        exact_override = override
      else:
        close_overrides.append(override)

    return struct(
        exact_override = exact_override,
        close_overrides = close_overrides
    )

def _handle_near_overrides(package, close_overrides):
    """
    Prints a message for the builder if near overrides are present, to allow them to fix
    potentially desync'd overrides after a cargo package upgrade

    Returns nothing
    """
    if not close_overrides:
      return

    close_override_versions = [override.pkg_version for override in close_overrides]
    print(("Did not find an exact override match for {}-{}, but found versions {}."
          + " Consider reviewing your CargoOverrides.bzl if you recently ran cargo-raze.")
          .format(package.pkg_name, package.pkg_version, close_override_versions))


def _generate_build_script_targets(name, srcs, target, crate_bzl, workspace_path, platform):
    """
    Generates bazel build targets for a build script to be executed prior to building the library.

    The exact targets generated are a rust_binary, which has both the normal dependencies and the build_dependencies,
    and a genrule, which executes the binary in a hermetic environment. Any generated files are linked into the
    crate's rust_library rule.

    Returns nothing
    """

    # TODO: Many build scripts depend on cargo-supplied environment variables
    # Unsure how to handle this.
    deps = _extract_dependency_paths(crate_bzl.dependencies, workspace_path) + _extract_dependency_paths(crate_bzl.build_dependencies, workspace_path)
    rust_binary(
        name = name + "_build_script",
        srcs = srcs,
        crate_root = target.path,
        deps = deps,
        rustc_flags = [
            "--cap-lints allow",
        ],
        crate_features = crate_bzl.features
    )

    native.genrule(
        name = name + "_build_script_executor",
        # TODO: This may not play nice with source_replacement
        srcs = srcs,
        outs = [name + "_out_dir_outputs.tar.gz"],
        tools = [":" + name + "_build_script"],
        cmd = "mkdir " + name + "_out_dir_outputs/;"
            + " (export CARGO_MANIFEST_DIR=\"$$PWD/" + workspace_path[2:] + "vendor/" + crate_bzl.package.pkg_name + '-' + crate_bzl.package.pkg_version + "\";"
            + " export TARGET='{}';".format(platform.triple)
            + " export RUST_BACKTRACE=1;"
            + " export OUT_DIR=$$PWD/" + name +  "_out_dir_outputs;"
            + " export BINARY_PATH=\"$$PWD/$(location :" + name + "_build_script)\";"
            + " export OUT_TAR=$$PWD/$@;"
            + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
    )


def _generate_lib_targets(name, srcs, crate_bzl, target, build_script_target, workspace_path):
    """
    Generates a rust_library build target for the cargo crate.

    Any output files from a build script will be linked in as compile time artifacts.

    Returns nothing
    """

    deps = _extract_dependency_paths(crate_bzl.dependencies, workspace_path)
    out_dir_tar = None
    if build_script_target:
        out_dir_tar = ":" + name + "_build_script_executor"

    target_name = target.name.replace('-', '_')

    # Refer to rust_library by desired (target) name, as users will expect it to `extern` by that name
    # However, create an alias to the "default" name, so we can refer to it globally
    if name != target_name:
        native.alias(name = name, actual = ":" + target_name)

    rust_library(
        name = target_name,
        srcs = srcs,
        crate_root = target.path,
        deps = deps,
        rustc_flags = [
            "--cap-lints allow",
        ],
        out_dir_tar = out_dir_tar,
        crate_features = crate_bzl.features
    )


def cargo_library(srcs, crate_bzl, cargo_override_bzl, platform, workspace_path="//"):

    # Gather list of nearly matching and exactly matching overrides
    override_details = _resolve_overrides(crate_bzl, cargo_override_bzl)
    exact_override = override_details.exact_override

    if not exact_override:
      _handle_near_overrides(crate_bzl.package, override_details.close_overrides)

    target_replacement = exact_override and exact_override.target_replacement
    config_replacement = exact_override and exact_override.config_replacement
    source_replacement = exact_override and exact_override.source_replacement

    if config_replacement:
        # TODO: Assert that config_replacement is a struct
        crate_bzl = config_replacement

    if target_replacement:
        # TODO: Assert that target_replacement is a string
        native.alias(
            name = crate_bzl.package.pkg_name.replace('-', '_'),
            actual = target_replacement,
        )

        # No point generating anything else -- we're ditching the generated targets
        return

    if source_replacement:
        # TODO: Assert that target_replacement is an array
        srcs = source_replacement

    name = crate_bzl.package.pkg_name.replace('-', '_')

    lib_target = None
    build_script_target = None

    for target in crate_bzl.targets:
        if "lib" in target.kinds:
            if lib_target:
                fail("{}-{} has more than one target with kind 'lib'", name, crate_bzl.package.pkg_version)
            lib_target = target

        if "custom-build" in target.kinds:
            if lib_target:
                fail("{}-{} has more than one target with kind 'custom-build'", name, crate_bzl.package.pkg_version)
            build_script_target = target

    if not lib_target:
        fail("{}-{} had no target with kind 'lib'", name, crate_bzl.package.pkg_version)

    if build_script_target:
      _generate_build_script_targets(
          name,
          srcs + native.glob(["*"]),
          target,
          crate_bzl,
          workspace_path,
          platform
      )

    _generate_lib_targets(name, srcs, crate_bzl, lib_target, build_script_target, workspace_path)
