"""
cargo-raze generated details for thread-id-3.1.0.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "thread-id",
        pkg_version = "3.1.0",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    metadeps = [],
    dependencies = [
        struct(
            name = "libc",
            version = "0.2.24",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "thread-id",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)
