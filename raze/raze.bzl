load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_binary")

def _contains_build_script(crate_bzl):
    for target in crate_bzl.targets:
        for kind in target.kinds:
          if kind == 'custom-build':
                return True

    return False

def _extract_dependency_paths(dependencies, workspace_path):
    deps = []
    for dependency in dependencies:
        dependency_name_sanitized = dependency.name.replace('-', '_')
        deps.append(workspace_path + "vendor/" + dependency.name + '-' + dependency.version + ":" + dependency_name_sanitized)
    return deps

def cargo_library(srcs, crate_bzl, cargo_override_bzl, platform, workspace_path="//"):

    name = crate_bzl.package.pkg_name.replace('-', '_')
    package = crate_bzl.package

    # Gather list of nearly matching and exactly matching overrides
    this_override = None
    close_overrides = []
    for override in cargo_override_bzl.dependency_overrides:
      if package.pkg_name != override.pkg_name:
        continue
      if package.pkg_version == override.pkg_version:
        if not package:
          fail("Package was already set once!")
        this_override = override
      else:
        close_overrides.append(override)

    if close_overrides and not this_override:
      close_override_versions = [override.pkg_version for override in close_overrides]
      print(("Did not find an exact override match for {}-{}, but found versions {}."
            + " Consider reviewing your CargoOverrides.bzl if you recently ran cargo-raze.")
            .format(package.pkg_name, package.pkg_version, close_override_versions))

    if this_override and this_override.target_replacement:
      print("Override was present, using it!")
      native.alias(
          name = name,
          actual = this_override.target
      )
      return

    if this_override and this_override.config_replacement:
      print("A config replacement was detected for {}-{}, but the feature is currently unimplemented".format(package.pkg_name, package.pkg_version))


    contains_build_script = _contains_build_script(crate_bzl)

    for target in crate_bzl.targets:
        if "lib" in target.kinds:
            deps = _extract_dependency_paths(crate_bzl.dependencies, workspace_path)
            full_srcs = srcs
            out_dir_tar = None
            if contains_build_script:
              out_dir_tar = ":" + name + "_build_script_executor"

            target_name = target.name.replace('-', '_')

            # Refer to rust_library by desired (target) name, as users will expect it to `extern` by that name
            # However, create an alias to the "default" name, so we can refer to it globally
            if name != target_name:
              native.alias(name = name, actual = ":" + target_name)

            rust_library(
                name = target_name,
                srcs = full_srcs,
                crate_root = target.path,
                deps = deps,
                rustc_flags = [
                    "--cap-lints allow",
                ],
                out_dir_tar = out_dir_tar,
                crate_features = crate_bzl.features
            )

        if "custom-build" in target.kinds:
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
                srcs = srcs + native.glob(["*"]),
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

