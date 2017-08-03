"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//complicated_cargo_library:__subpackages__"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

rust_library(
    name = "regex_arm_unknown_linux_musleabi",
    crate_root = "src/lib.rs",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = ["//complicated_cargo_library/vendor/aho-corasick-0.6.3:aho_corasick_arm_unknown_linux_musleabi", "//complicated_cargo_library/vendor/memchr-1.0.1:memchr_arm_unknown_linux_musleabi", "//complicated_cargo_library/vendor/regex-syntax-0.4.1:regex_syntax_arm_unknown_linux_musleabi", "//complicated_cargo_library/vendor/thread_local-0.3.3:thread_local_arm_unknown_linux_musleabi", "//complicated_cargo_library/vendor/utf8-ranges-1.0.0:utf8_ranges_arm_unknown_linux_musleabi"],
    rustc_flags = [
        "--cap-lints allow",
        "--target=arm-unknown-linux-musleabi"
    ],
    out_dir_tar = None,
    crate_features = [],
)