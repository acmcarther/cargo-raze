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
    name = "toml",
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
    name = "toml2json",
    crate_root = "examples/toml2json.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":toml",
    
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu"
    ],
    
    crate_features = [
      
    ],
)
rust_test(
    name = "formatting",
    crate_root = "tests/formatting.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":toml",
    
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
)
rust_test(
    name = "invalid",
    crate_root = "tests/invalid.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":toml",
    
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
)
rust_test(
    name = "valid",
    crate_root = "tests/valid.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":toml",
    
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
)
