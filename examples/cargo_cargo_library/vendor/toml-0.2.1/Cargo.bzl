"""
cargo-raze generated details for toml-0.2.1.

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
        pkg_name = "toml",
        pkg_version = "0.2.1",
    ),
    dependencies = [
        struct(
            name = "rustc-serialize",
            version = "0.3.24",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [
        struct(
            name = "rustc-serialize",
            version = "0.3.24",
        ),
    ],
    features = [
        "rustc-serialize",
        "default",
    ],
    targets = [
        struct(
            name = "toml",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "toml2json",
            kinds = [
                "example",
            ],
            path = "examples/toml2json.rs",
        ),
        struct(
            name = "invalid",
            kinds = [
                "test",
            ],
            path = "tests/invalid.rs",
        ),
        struct(
            name = "formatting",
            kinds = [
                "test",
            ],
            path = "tests/formatting.rs",
        ),
        struct(
            name = "valid",
            kinds = [
                "test",
            ],
            path = "tests/valid.rs",
        ),
    ],
)
