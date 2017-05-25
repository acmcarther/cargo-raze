"""
cargo-raze generated details for openssl-0.9.12.

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
        pkg_name = "openssl",
        pkg_version = "0.9.12",
    ),
    dependencies = [
        struct(
            name = "libc",
            version = "0.2.23",
        ),
        struct(
            name = "lazy_static",
            version = "0.2.8",
        ),
        struct(
            name = "foreign-types",
            version = "0.2.0",
        ),
        struct(
            name = "bitflags",
            version = "0.8.2",
        ),
        struct(
            name = "openssl-sys",
            version = "0.9.12",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "openssl",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "build-script-build",
            kinds = [
                "custom-build",
            ],
            path = "build.rs",
        ),
    ],
)
