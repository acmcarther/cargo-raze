"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//complicated_cargo_library:__subpackages__"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

rust_library(
    name = "pkg_config_arm_unknown_linux_musleabi",
    crate_root = "src/lib.rs",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [],
    rustc_flags = [
        "--cap-lints allow",
    ],
    out_dir_tar = None,
    crate_features = [],
)