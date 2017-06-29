"""
cargo-raze vendor-wide workspace file

DO NOT EDIT! Replaced on runs of cargo-raze
"""

workspace = struct(
    platform = struct(
        triple = "x86_64-unknown-linux-gnu",
        flags = [
            "debug_assertions",
            "target_thread_local",
            "unix",
        ],
        vars = [
            (
                "target_arch",
                "x86_64",
            ),
            (
                "target_endian",
                "little",
            ),
            (
                "target_env",
                "gnu",
            ),
            (
                "target_family",
                "unix",
            ),
            (
                "target_feature",
                "sse",
            ),
            (
                "target_feature",
                "sse2",
            ),
            (
                "target_has_atomic",
                "16",
            ),
            (
                "target_has_atomic",
                "32",
            ),
            (
                "target_has_atomic",
                "64",
            ),
            (
                "target_has_atomic",
                "8",
            ),
            (
                "target_has_atomic",
                "ptr",
            ),
            (
                "target_os",
                "linux",
            ),
            (
                "target_pointer_width",
                "64",
            ),
            (
                "target_vendor",
                "unknown",
            ),
        ],
    ),
    packages = [
        struct(
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
        ),
        struct(
            package = struct(
                pkg_name = "error-chain",
                pkg_version = "0.10.0",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "all",
                    kinds = [
                        "example",
                    ],
                    path = "examples/all.rs",
                ),
                struct(
                    name = "doc",
                    kinds = [
                        "example",
                    ],
                    path = "examples/doc.rs",
                ),
                struct(
                    name = "error-chain",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
                struct(
                    name = "quick_main",
                    kinds = [
                        "test",
                    ],
                    path = "tests/quick_main.rs",
                ),
                struct(
                    name = "quickstart",
                    kinds = [
                        "example",
                    ],
                    path = "examples/quickstart.rs",
                ),
                struct(
                    name = "size",
                    kinds = [
                        "example",
                    ],
                    path = "examples/size.rs",
                ),
                struct(
                    name = "tests",
                    kinds = [
                        "test",
                    ],
                    path = "tests/tests.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "kernel32-sys",
                pkg_version = "0.2.2",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [
                struct(
                    name = "winapi",
                    version = "0.2.8",
                ),
            ],
            build_dependencies = [
                struct(
                    name = "winapi-build",
                    version = "0.1.1",
                ),
            ],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "build-script-build",
                    kinds = [
                        "custom-build",
                    ],
                    path = "build.rs",
                ),
                struct(
                    name = "kernel32",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "libc",
                pkg_version = "0.2.24",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [],
            build_dependencies = [],
            dev_dependencies = [],
            features = [
                "default",
                "use_std",
            ],
            targets = [
                struct(
                    name = "libc",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "memchr",
                pkg_version = "1.0.1",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [
                struct(
                    name = "libc",
                    version = "0.2.24",
                ),
            ],
            build_dependencies = [],
            dev_dependencies = [],
            features = [
                "default",
                "libc",
                "use_std",
            ],
            targets = [
                struct(
                    name = "bench",
                    kinds = [
                        "bench",
                    ],
                    path = "benches/bench.rs",
                ),
                struct(
                    name = "memchr",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "metadeps",
                pkg_version = "1.1.2",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [
                struct(
                    name = "error-chain",
                    version = "0.10.0",
                ),
                struct(
                    name = "pkg-config",
                    version = "0.3.9",
                ),
                struct(
                    name = "toml",
                    version = "0.2.1",
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
        ),
        struct(
            package = struct(
                pkg_name = "pkg-config",
                pkg_version = "0.3.9",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "pkg-config",
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
        ),
        struct(
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
        ),
        struct(
            package = struct(
                pkg_name = "regex-syntax",
                pkg_version = "0.4.1",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "regex-syntax",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "thread-id",
                pkg_version = "3.1.0",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [
                struct(
                    name = "libc",
                    version = "0.2.24",
                ),
            ],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "thread-id",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "thread_local",
                pkg_version = "0.3.3",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [
                struct(
                    name = "thread-id",
                    version = "3.1.0",
                ),
                struct(
                    name = "unreachable",
                    version = "0.1.1",
                ),
            ],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "thread_local",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "toml",
                pkg_version = "0.2.1",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "formatting",
                    kinds = [
                        "test",
                    ],
                    path = "tests/formatting.rs",
                ),
                struct(
                    name = "invalid",
                    kinds = [
                        "test",
                    ],
                    path = "tests/invalid.rs",
                ),
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
                    name = "valid",
                    kinds = [
                        "test",
                    ],
                    path = "tests/valid.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "unreachable",
                pkg_version = "0.1.1",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [
                struct(
                    name = "void",
                    version = "1.0.2",
                ),
            ],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "unreachable",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "utf8-ranges",
                pkg_version = "1.0.0",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "bench",
                    kinds = [
                        "bench",
                    ],
                    path = "benches/bench.rs",
                ),
                struct(
                    name = "utf8-ranges",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "void",
                pkg_version = "1.0.2",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "void",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "winapi",
                pkg_version = "0.2.8",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "winapi",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "winapi-build",
                pkg_version = "0.1.1",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "build",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "x11",
                pkg_version = "2.14.0",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [
                struct(
                    name = "libc",
                    version = "0.2.24",
                ),
            ],
            build_dependencies = [
                struct(
                    name = "metadeps",
                    version = "1.1.2",
                ),
            ],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "build-script-build",
                    kinds = [
                        "custom-build",
                    ],
                    path = "build.rs",
                ),
                struct(
                    name = "hello-world",
                    kinds = [
                        "example",
                    ],
                    path = "examples/hello-world.rs",
                ),
                struct(
                    name = "input",
                    kinds = [
                        "example",
                    ],
                    path = "examples/input.rs",
                ),
                struct(
                    name = "x11",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
                struct(
                    name = "xrecord",
                    kinds = [
                        "example",
                    ],
                    path = "examples/xrecord.rs",
                ),
            ],
        ),
    ],
)
