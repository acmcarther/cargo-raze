load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library",)

def cargo_library(srcs, cargo_bzl, cargo_override_bzl, workspace_path="//vendor/"):
    # Ignoring cargo_override_bzl for now
    name = cargo_bzl.package.pkg_name.replace('-', '_')

    for target in cargo_bzl.targets:
        if "lib" not in target.kinds:
            # Other kinds of targets are unsupported
            continue
        deps = []
        for dependency in cargo_bzl.dependencies:
            dependency_name_sanitized = dependency.name.replace('-', '_')
            deps.append(workspace_path + dependency.name + '-' + dependency.version + ":" + dependency_name_sanitized)

        # Ignoring other build targets -- focused on the lib
        rust_library(
            name = name,
            srcs = srcs,
            crate_root = target.path,
            deps = deps,
            rustc_flags = [
                "--cap-lints allow",
            ],
            crate_features = cargo_bzl.features
        )
