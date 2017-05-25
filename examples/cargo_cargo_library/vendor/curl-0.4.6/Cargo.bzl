"""
cargo-raze generated details for curl-0.4.6.

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
        pkg_name = "curl",
        pkg_version = "0.4.6",
    ),
    dependencies = [
        struct(
            name = "libc",
            version = "0.2.23",
        ),
        struct(
            name = "curl-sys",
            version = "0.3.11",
        ),
        struct(
            name = "openssl-probe",
            version = "0.1.1",
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
            name = "curl",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "easy",
            kinds = [
                "test",
            ],
            path = "tests/easy.rs",
        ),
        struct(
            name = "post",
            kinds = [
                "test",
            ],
            path = "tests/post.rs",
        ),
        struct(
            name = "multi",
            kinds = [
                "test",
            ],
            path = "tests/multi.rs",
        ),
    ],
)
