"""
cargo-raze generated details for unreachable-0.1.1.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "unreachable",
        pkg_version = "0.1.1",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    metadeps = [],
    dependencies = [
        struct(
            name = "void",
            version = "1.0.2",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "unreachable",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)
