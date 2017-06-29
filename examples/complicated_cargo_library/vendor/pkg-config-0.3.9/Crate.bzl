"""
cargo-raze generated details for pkg-config-0.3.9.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "pkg-config",
        pkg_version = "0.3.9",
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
            name = "pkg-config",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "test",
            kinds = [
                "test",
            ],
            path = "tests/test.rs",
        ),
    ],
)
