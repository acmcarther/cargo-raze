"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//complicated_cargo_library:__subpackages__"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

rust_binary(
    name = x11_arm_unknown_linux_musleabi_build_script
    srcs = glob(["*", "src/**/*.rs"]),
    crate_root = "build.rs",
    deps = ["//complicated_cargo_library/vendor/libc-0.2.24:libc_arm_unknown_linux_musleabi", "//complicated_cargo_library/vendor/metadeps-1.1.2:metadeps"],
    rustc_flags = [
        "--cap-lints allow",
        "--target=arm-unknown-linux-musleabi"
    ],
    crate_features = [],
)
genrule(
    name = x11_arm_unknown_linux_musleabi_build_script_executor
    srcs = glob(["*", "src/**/*.rs"]),
    outs = ["x11_arm_unknown_linux_musleabi_out_dir_outputs.tar.gz"],
    tools = [":x11_arm_unknown_linux_musleabi_build_script"],
    cmd = "mkdir x11_arm_unknown_linux_musleabi_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/complicated_cargo_library/vendor/x11-2.14.0\";"
        + " export TARGET='arm-unknown-linux-musleabi';"
        + " export RUST_BACKTRACE=1;"
        + " export OUT_DIR=$$PWD/x11_arm_unknown_linux_musleabi_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :x11_arm_unknown_linux_musleabi_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)
rust_library(
    name = "x11_arm_unknown_linux_musleabi",
    crate_root = "src/lib.rs",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = ["//complicated_cargo_library/vendor/libc-0.2.24:libc_arm_unknown_linux_musleabi"],
    rustc_flags = [
        "--cap-lints allow",
        "--target=arm-unknown-linux-musleabi"
    ],
    out_dir_tar = ":x11_arm_unknown_linux_musleabi_build_script_executor",
    crate_features = [],
)