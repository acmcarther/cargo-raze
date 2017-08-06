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
    name = "metadeps",
    crate_root = "src/lib.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    "//complicated_cargo_library/vendor/pkg-config-0.3.9:pkg_config",
    "//complicated_cargo_library/vendor/toml-0.2.1:toml",
    "//complicated_cargo_library/vendor/error-chain-0.10.0:error_chain",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
)
rust_test(
    name = "test",
    crate_root = "tests/test.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":metadeps",
    "//complicated_cargo_library/vendor/pkg-config-0.3.9:pkg_config",
    "//complicated_cargo_library/vendor/toml-0.2.1:toml",
    "//complicated_cargo_library/vendor/error-chain-0.10.0:error_chain",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      
    ],
)
