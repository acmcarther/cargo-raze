"""
cargo-raze generated details for git2-curl-0.7.0.

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
        pkg_name = "git2-curl",
        pkg_version = "0.7.0",
    ),
    dependencies = [
        struct(
            name = "url",
            version = "1.4.0",
        ),
        struct(
            name = "log",
            version = "0.3.8",
        ),
        struct(
            name = "git2",
            version = "0.6.5",
        ),
        struct(
            name = "curl",
            version = "0.4.6",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "git2-curl",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "all",
            kinds = [
                "test",
            ],
            path = "tests/all.rs",
        ),
    ],
)
