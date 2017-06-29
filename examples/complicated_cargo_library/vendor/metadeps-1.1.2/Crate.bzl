"""
cargo-raze generated details for metadeps-1.1.2.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "metadeps",
        pkg_version = "1.1.2",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    metadeps = [],
    dependencies = [
        struct(
            name = "error-chain",
            version = "0.10.0",
        ),
        struct(
            name = "pkg-config",
            version = "0.3.9",
        ),
        struct(
            name = "toml",
            version = "0.2.1",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "metadeps",
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
