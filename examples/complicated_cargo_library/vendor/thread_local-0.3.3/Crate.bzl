"""
cargo-raze generated details for thread_local-0.3.3.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "thread_local",
        pkg_version = "0.3.3",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
        struct(
            name = "unreachable",
            version = "0.1.1",
        ),
        struct(
            name = "thread-id",
            version = "3.1.0",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "thread_local",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)
