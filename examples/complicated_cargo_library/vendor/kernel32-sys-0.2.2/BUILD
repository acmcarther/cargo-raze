"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//complicated_cargo_library:__subpackages__"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

rust_binary(
    name = kernel32_sys_arm_unknown_linux_musleabi_build_script
    srcs = glob(["*", "src/**/*.rs"]),
    crate_root = "build.rs",
    deps = ["//complicated_cargo_library/vendor/winapi-0.2.8:winapi", "//complicated_cargo_library/vendor/winapi-build-0.1.1:winapi_build"],
    rustc_flags = [
        "--cap-lints allow",
    ],
    crate_features = [],
)
genrule(
    name = kernel32_sys_arm_unknown_linux_musleabi_build_script_executor
    srcs = glob(["*", "src/**/*.rs"]),
    outs = ["kernel32_sys_arm_unknown_linux_musleabi_out_dir_outputs.tar.gz"],
    tools = [":kernel32_sys_arm_unknown_linux_musleabi_build_script"],
    cmd = "mkdir kernel32_sys_arm_unknown_linux_musleabi_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWD/complicated_cargo_library/vendor/kernel32-sys-0.2.2\";"
        + " export TARGET='arm-unknown-linux-musleabi';"
        + " export RUST_BACKTRACE=1;"
        + " export OUT_DIR=$$PWD/kernel32_sys_arm_unknown_linux_musleabi_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :kernel32_sys_arm_unknown_linux_musleabi_build_script)\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)
alias(
    name = "kernel32_sys_arm_unknown_linux_musleabi",
    actual = ":kernel32_arm_unknown_linux_musleabi",
)
rust_library(
    name = "kernel32_arm_unknown_linux_musleabi",
    crate_root = "src/lib.rs",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = ["//complicated_cargo_library/vendor/winapi-0.2.8:winapi"],
    rustc_flags = [
        "--cap-lints allow",
    ],
    out_dir_tar = "kernel32_sys_arm_unknown_linux_musleabi_build_script_executor",
    crate_features = [],
)