"""
cargo-raze generated details for flate2-0.2.19.

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
        pkg_name = "flate2",
        pkg_version = "0.2.19",
    ),
    dependencies = [
        struct(
            name = "miniz-sys",
            version = "0.1.9",
        ),
        struct(
            name = "libc",
            version = "0.2.23",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [
        "default",
        "miniz-sys",
    ],
    targets = [
        struct(
            name = "flate2",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "gunzip",
            kinds = [
                "test",
            ],
            path = "tests/gunzip.rs",
        ),
        struct(
            name = "tokio",
            kinds = [
                "test",
            ],
            path = "tests/tokio.rs",
        ),
    ],
)
