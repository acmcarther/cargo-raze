"""
cargo-raze generated details for miow-0.1.5.

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
        pkg_name = "miow",
        pkg_version = "0.1.5",
    ),
    dependencies = [
        struct(
            name = "winapi",
            version = "0.2.8",
        ),
        struct(
            name = "net2",
            version = "0.2.29",
        ),
        struct(
            name = "kernel32-sys",
            version = "0.2.2",
        ),
        struct(
            name = "ws2_32-sys",
            version = "0.2.1",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "miow",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)
