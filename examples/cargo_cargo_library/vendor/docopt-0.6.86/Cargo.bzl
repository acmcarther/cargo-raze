"""
cargo-raze generated details for docopt-0.6.86.

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
        pkg_name = "docopt",
        pkg_version = "0.6.86",
    ),
    dependencies = [
        struct(
            name = "strsim",
            version = "0.5.2",
        ),
        struct(
            name = "rustc-serialize",
            version = "0.3.24",
        ),
        struct(
            name = "regex",
            version = "0.1.80",
        ),
        struct(
            name = "lazy_static",
            version = "0.2.8",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "docopt",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "docopt-wordlist",
            kinds = [
                "bin",
            ],
            path = "src/wordlist.rs",
        ),
        struct(
            name = "decode",
            kinds = [
                "example",
            ],
            path = "examples/decode.rs",
        ),
        struct(
            name = "optional_command",
            kinds = [
                "example",
            ],
            path = "examples/optional_command.rs",
        ),
        struct(
            name = "cargo",
            kinds = [
                "example",
            ],
            path = "examples/cargo.rs",
        ),
        struct(
            name = "verbose_multiple",
            kinds = [
                "example",
            ],
            path = "examples/verbose_multiple.rs",
        ),
        struct(
            name = "hashmap",
            kinds = [
                "example",
            ],
            path = "examples/hashmap.rs",
        ),
        struct(
            name = "cp",
            kinds = [
                "example",
            ],
            path = "examples/cp.rs",
        ),
    ],
)
