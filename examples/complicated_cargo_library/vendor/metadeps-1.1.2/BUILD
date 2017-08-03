"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//complicated_cargo_library:__subpackages__"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

rust_library(
    name = "metadeps_arm_unknown_linux_musleabi",
    crate_root = "src/lib.rs",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = ["//complicated_cargo_library/vendor/error-chain-0.10.0:error_chain", "//complicated_cargo_library/vendor/pkg-config-0.3.9:pkg_config", "//complicated_cargo_library/vendor/toml-0.2.1:toml"],
    rustc_flags = [
        "--cap-lints allow",
    ],
    out_dir_tar = None,
    crate_features = [],
)