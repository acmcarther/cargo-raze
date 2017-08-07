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

rust_binary(
    name = "x11_build_script",
    srcs = glob(["*", "**/*.rs"]),
    crate_root = "build.rs",
    deps = [
      "//complicated_cargo_library/vendor/metadeps-1.1.2:metadeps",
    ]
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    visibility = ["//visibility:private"],
)

genrule(
    name = "x11_build_script_executor",
    srcs = glob(["*", "**/*.rs"]),
    outs = ["x11_out_dir_outputs.tar.gz"],
    tools = [":x11_build_script"],
    cmd = "mkdir x11_out_dir_outputs/;"
        + " (export CARGO_MANIFEST_DIR=\"$$PWDcomplicated_cargo_library/vendor/x11-2.14.0\";"
        + " export TARGET='x86_64-unknown-linux-gnu';"
        + " export RUST_BACKTRACE=1;"
        + " export OUT_DIR=$$PWD/x11_out_dir_outputs;"
        + " export BINARY_PATH=\"$$PWD/$(location :x11_build_script\";"
        + " export OUT_TAR=$$PWD/$@;"
        + " cd $$(dirname $(location :Cargo.toml)) && $$BINARY_PATH && tar -czf $$OUT_TAR -C $$OUT_DIR .)"
)
# NOTE: Example may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_binary(
    name = "hello_world",
    crate_root = "examples/hello-world.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":x11",
    "//complicated_cargo_library/vendor/libc-0.2.24:libc",
    
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
    name = "input",
    crate_root = "examples/input.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":x11",
    "//complicated_cargo_library/vendor/libc-0.2.24:libc",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
rust_library(
    name = "x11",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    "//complicated_cargo_library/vendor/libc-0.2.24:libc",
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
    name = "xrecord",
    crate_root = "examples/xrecord.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":x11",
    "//complicated_cargo_library/vendor/libc-0.2.24:libc",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
    visibility = ["//visibility:private"],
)
