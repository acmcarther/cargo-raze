"""
cargo-raze generated details for aho-corasick-0.6.3.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "aho-corasick",
        pkg_version = "0.6.3",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    metadeps = [],
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
            name = "aho-corasick-dot",
            kinds = [
                "bin",
            ],
            path = "src/main.rs",
        ),
        struct(
            name = "aho_corasick",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "bench",
            kinds = [
                "bench",
            ],
            path = "benches/bench.rs",
        ),
        struct(
            name = "dict-search",
            kinds = [
                "example",
            ],
            path = "examples/dict-search.rs",
        ),
    ],
)
