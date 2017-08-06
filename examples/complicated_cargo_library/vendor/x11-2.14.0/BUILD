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
# Unsupported target "build-script-build" with type "custom-build" omitted
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
)
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
)
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
)
