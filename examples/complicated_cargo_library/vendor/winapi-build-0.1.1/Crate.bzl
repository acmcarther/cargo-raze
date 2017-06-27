"""
cargo-raze generated details for winapi-build-0.1.1.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "winapi-build",
        pkg_version = "0.1.1",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "build",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)
