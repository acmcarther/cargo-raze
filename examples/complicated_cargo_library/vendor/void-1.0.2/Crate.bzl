"""
cargo-raze generated details for void-1.0.2.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "void",
        pkg_version = "1.0.2",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    metadeps = [],
    dependencies = [],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "void",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)
