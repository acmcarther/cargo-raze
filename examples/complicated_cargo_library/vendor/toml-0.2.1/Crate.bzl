"""
cargo-raze generated details for toml-0.2.1.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "toml",
        pkg_version = "0.2.1",
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
            name = "formatting",
            kinds = [
                "test",
            ],
            path = "tests/formatting.rs",
        ),
        struct(
            name = "invalid",
            kinds = [
                "test",
            ],
            path = "tests/invalid.rs",
        ),
        struct(
            name = "toml",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "toml2json",
            kinds = [
                "example",
            ],
            path = "examples/toml2json.rs",
        ),
        struct(
            name = "valid",
            kinds = [
                "test",
            ],
            path = "tests/valid.rs",
        ),
    ],
)
