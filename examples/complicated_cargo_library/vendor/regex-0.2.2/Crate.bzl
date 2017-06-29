"""
cargo-raze generated details for regex-0.2.2.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "regex",
        pkg_version = "0.2.2",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    metadeps = [],
    dependencies = [
        struct(
            name = "aho-corasick",
            version = "0.6.3",
        ),
        struct(
            name = "memchr",
            version = "1.0.1",
        ),
        struct(
            name = "regex-syntax",
            version = "0.4.1",
        ),
        struct(
            name = "thread_local",
            version = "0.3.3",
        ),
        struct(
            name = "utf8-ranges",
            version = "1.0.0",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "backtrack",
            kinds = [
                "test",
            ],
            path = "tests/test_backtrack.rs",
        ),
        struct(
            name = "backtrack-bytes",
            kinds = [
                "test",
            ],
            path = "tests/test_backtrack_bytes.rs",
        ),
        struct(
            name = "backtrack-utf8bytes",
            kinds = [
                "test",
            ],
            path = "tests/test_backtrack_utf8bytes.rs",
        ),
        struct(
            name = "bug347",
            kinds = [
                "example",
            ],
            path = "examples/bug347.rs",
        ),
        struct(
            name = "default",
            kinds = [
                "test",
            ],
            path = "tests/test_default.rs",
        ),
        struct(
            name = "default-bytes",
            kinds = [
                "test",
            ],
            path = "tests/test_default_bytes.rs",
        ),
        struct(
            name = "nfa",
            kinds = [
                "test",
            ],
            path = "tests/test_nfa.rs",
        ),
        struct(
            name = "nfa-bytes",
            kinds = [
                "test",
            ],
            path = "tests/test_nfa_bytes.rs",
        ),
        struct(
            name = "nfa-utf8bytes",
            kinds = [
                "test",
            ],
            path = "tests/test_nfa_utf8bytes.rs",
        ),
        struct(
            name = "regex",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "shootout-regex-dna",
            kinds = [
                "example",
            ],
            path = "examples/shootout-regex-dna.rs",
        ),
        struct(
            name = "shootout-regex-dna-bytes",
            kinds = [
                "example",
            ],
            path = "examples/shootout-regex-dna-bytes.rs",
        ),
        struct(
            name = "shootout-regex-dna-cheat",
            kinds = [
                "example",
            ],
            path = "examples/shootout-regex-dna-cheat.rs",
        ),
        struct(
            name = "shootout-regex-dna-replace",
            kinds = [
                "example",
            ],
            path = "examples/shootout-regex-dna-replace.rs",
        ),
        struct(
            name = "shootout-regex-dna-single",
            kinds = [
                "example",
            ],
            path = "examples/shootout-regex-dna-single.rs",
        ),
        struct(
            name = "shootout-regex-dna-single-cheat",
            kinds = [
                "example",
            ],
            path = "examples/shootout-regex-dna-single-cheat.rs",
        ),
        struct(
            name = "shootout-regex-redux",
            kinds = [
                "example",
            ],
            path = "examples/shootout-regex-redux.rs",
        ),
        struct(
            name = "shootout-regex-redux-1",
            kinds = [
                "example",
            ],
            path = "examples/shootout-regex-redux-1.rs",
        ),
        struct(
            name = "shootout-regex-redux-chunked",
            kinds = [
                "example",
            ],
            path = "examples/shootout-regex-redux-chunked.rs",
        ),
    ],
)
