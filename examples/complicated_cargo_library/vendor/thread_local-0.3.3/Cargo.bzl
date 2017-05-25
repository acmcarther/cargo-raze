"""
cargo-raze generated details for thread_local-0.3.3.

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
        pkg_name = "thread_local",
        pkg_version = "0.3.3",
    ),
    dependencies = [
        struct(
            name = "thread-id",
            version = "3.1.0",
        ),
        struct(
            name = "unreachable",
            version = "0.1.1",
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
