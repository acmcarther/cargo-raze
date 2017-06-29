"""
cargo-raze generated details for error-chain-0.10.0.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "error-chain",
        pkg_version = "0.10.0",
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
            name = "all",
            kinds = [
                "example",
            ],
            path = "examples/all.rs",
        ),
        struct(
            name = "doc",
            kinds = [
                "example",
            ],
            path = "examples/doc.rs",
        ),
        struct(
            name = "error-chain",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "quick_main",
            kinds = [
                "test",
            ],
            path = "tests/quick_main.rs",
        ),
        struct(
            name = "quickstart",
            kinds = [
                "example",
            ],
            path = "examples/quickstart.rs",
        ),
        struct(
            name = "size",
            kinds = [
                "example",
            ],
            path = "examples/size.rs",
        ),
        struct(
            name = "tests",
            kinds = [
                "test",
            ],
            path = "tests/tests.rs",
        ),
    ],
)
