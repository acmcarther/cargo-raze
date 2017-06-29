"""
cargo-raze generated details for fern-0.3.5.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "fern",
        pkg_version = "0.3.5",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    metadeps = [],
    dependencies = [
        struct(
            name = "log",
            version = "0.3.7",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "doc_test_copy",
            kinds = [
                "test",
            ],
            path = "tests/doc_test_copy.rs",
        ),
        struct(
            name = "fern",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "lib",
            kinds = [
                "test",
            ],
            path = "tests/lib.rs",
        ),
    ],
)
