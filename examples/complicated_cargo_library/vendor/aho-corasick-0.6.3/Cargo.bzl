"""
cargo-raze generated details for aho-corasick-0.6.3.

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
        pkg_name = "aho-corasick",
        pkg_version = "0.6.3",
    ),
    dependencies = [
        struct(
            name = "memchr",
            version = "1.0.1",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "aho_corasick",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "aho-corasick-dot",
            kinds = [
                "bin",
            ],
            path = "src/main.rs",
        ),
        struct(
            name = "dict-search",
            kinds = [
                "example",
            ],
            path = "examples/dict-search.rs",
        ),
        struct(
            name = "bench",
            kinds = [
                "bench",
            ],
            path = "benches/bench.rs",
        ),
    ],
)
