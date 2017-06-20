"""
cargo-raze generated details for error-chain-0.10.0.

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
        pkg_name = "error-chain",
        pkg_version = "0.10.0",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    dependencies = [],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "error-chain",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "doc",
            kinds = [
                "example",
            ],
            path = "examples/doc.rs",
        ),
        struct(
            name = "size",
            kinds = [
                "example",
            ],
            path = "examples/size.rs",
        ),
        struct(
            name = "all",
            kinds = [
                "example",
            ],
            path = "examples/all.rs",
        ),
        struct(
            name = "quickstart",
            kinds = [
                "example",
            ],
            path = "examples/quickstart.rs",
        ),
        struct(
            name = "tests",
            kinds = [
                "test",
            ],
            path = "tests/tests.rs",
        ),
        struct(
            name = "quick_main",
            kinds = [
                "test",
            ],
            path = "tests/quick_main.rs",
        ),
    ],
)
