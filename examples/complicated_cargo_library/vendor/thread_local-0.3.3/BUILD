"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//complicated_cargo_library:__subpackages__"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

rust_library(
    name = "thread_local_arm_unknown_linux_musleabi",
    crate_root = "src/lib.rs",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = ["//complicated_cargo_library/vendor/thread-id-3.1.0:thread_id_arm_unknown_linux_musleabi", "//complicated_cargo_library/vendor/unreachable-0.1.1:unreachable_arm_unknown_linux_musleabi"],
    rustc_flags = [
        "--cap-lints allow",
        "--target=arm-unknown-linux-musleabi"
    ],
    out_dir_tar = None,
    crate_features = [],
)