"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//complicated_cargo_library:__subpackages__"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)


# NOTE: Test may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_test(
    name = "backtrack",
    crate_root = "tests/test_backtrack.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Test may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_test(
    name = "backtrack_bytes",
    crate_root = "tests/test_backtrack_bytes.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Test may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_test(
    name = "backtrack_utf8bytes",
    crate_root = "tests/test_backtrack_utf8bytes.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Example may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_binary(
    name = "bug347",
    crate_root = "examples/bug347.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Test may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_test(
    name = "default",
    crate_root = "tests/test_default.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Test may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_test(
    name = "default_bytes",
    crate_root = "tests/test_default_bytes.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Test may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_test(
    name = "nfa",
    crate_root = "tests/test_nfa.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Test may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_test(
    name = "nfa_bytes",
    crate_root = "tests/test_nfa_bytes.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Test may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_test(
    name = "nfa_utf8bytes",
    crate_root = "tests/test_nfa_utf8bytes.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
rust_library(
    name = "regex",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
)
# NOTE: Example may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_binary(
    name = "shootout_regex_dna",
    crate_root = "examples/shootout-regex-dna.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Example may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_binary(
    name = "shootout_regex_dna_bytes",
    crate_root = "examples/shootout-regex-dna-bytes.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Example may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_binary(
    name = "shootout_regex_dna_cheat",
    crate_root = "examples/shootout-regex-dna-cheat.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Example may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_binary(
    name = "shootout_regex_dna_replace",
    crate_root = "examples/shootout-regex-dna-replace.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Example may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_binary(
    name = "shootout_regex_dna_single",
    crate_root = "examples/shootout-regex-dna-single.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Example may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_binary(
    name = "shootout_regex_dna_single_cheat",
    crate_root = "examples/shootout-regex-dna-single-cheat.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Example may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_binary(
    name = "shootout_regex_redux",
    crate_root = "examples/shootout-regex-redux.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Example may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_binary(
    name = "shootout_regex_redux_1",
    crate_root = "examples/shootout-regex-redux-1.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
# NOTE: Example may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_binary(
    name = "shootout_regex_redux_chunked",
    crate_root = "examples/shootout-regex-redux-chunked.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":regex",
    "//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick",
    "//complicated_cargo_library/vendor/memchr-1.0.1:memchr",
    "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax",
    "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local",
    "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
