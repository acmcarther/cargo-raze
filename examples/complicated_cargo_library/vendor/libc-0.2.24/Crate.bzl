"""
cargo-raze generated details for libc-0.2.24.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "libc",
        pkg_version = "0.2.24",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [],
    build_dependencies = [],
    dev_dependencies = [],
    features = [
        "default",
        "use_std",
    ],
    targets = [
        struct(
            name = "libc",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)
