"""
cargo-raze generated details for metadeps-1.1.2.

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

DO NOT MODIFY! Instead, update vendor/CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "metadeps",
        pkg_version = "1.1.2",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [
        struct(
            name = "error-chain",
            version = "0.10.0",
        ),
        struct(
            name = "toml",
            version = "0.2.1",
        ),
        struct(
            name = "pkg-config",
            version = "0.3.9",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "metadeps",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "test",
            kinds = [
                "test",
            ],
            path = "tests/test.rs",
        ),
    ],
)
