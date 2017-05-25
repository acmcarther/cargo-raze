"""
cargo-raze generated details for semver-0.5.1.

Generated for:
platform_triple: x86_64-unknown-linux-gnu
platform_attrs:
[
    "debug_assertions",
    "target_arch: x86_64",
    "target_endian: little",
    "target_env: gnu",
    "target_family: unix",
    "target_feature: sse",
    "target_feature: sse2",
    "target_has_atomic: 16",
    "target_has_atomic: 32",
    "target_has_atomic: 64",
    "target_has_atomic: 8",
    "target_has_atomic: ptr",
    "target_os: linux",
    "target_pointer_width: 64",
    "target_thread_local",
    "target_vendor: unknown",
    "unix"
]

DO NOT MODIFY! Instead, add a CargoOverride.bzl mixin.
"""
description = struct(
    package = struct(
        pkg_name = "semver",
        pkg_version = "0.5.1",
    ),
    dependencies = [
        struct(
            name = "semver-parser",
            version = "0.6.2",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [
        "default",
    ],
    targets = [
        struct(
            name = "semver",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "deprecation",
            kinds = [
                "test",
            ],
            path = "tests/deprecation.rs",
        ),
        struct(
            name = "regression",
            kinds = [
                "test",
            ],
            path = "tests/regression.rs",
        ),
    ],
)
