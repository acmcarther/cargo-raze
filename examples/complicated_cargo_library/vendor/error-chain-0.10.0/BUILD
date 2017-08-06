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
    name = "error_chain",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
)
rust_binary(
    name = "quickstart",
    crate_root = "examples/quickstart.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":error_chain",
    
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
)
rust_binary(
    name = "doc",
    crate_root = "examples/doc.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":error_chain",
    
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
)
rust_binary(
    name = "all",
    crate_root = "examples/all.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":error_chain",
    
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
)
rust_binary(
    name = "size",
    crate_root = "examples/size.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":error_chain",
    
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
)
rust_test(
    name = "tests",
    crate_root = "tests/tests.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":error_chain",
    
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
)
rust_test(
    name = "quick_main",
    crate_root = "tests/quick_main.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":error_chain",
    
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
)
