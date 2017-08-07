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


# NOTE: Benchmark may not function correctly: Development dependencies will be absent
# TODO(acmcarther/cargo-raze/issues/7): Extend this with dev dependencies
rust_bench_test(
    name = "bench",
    crate_root = "benches/bench.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
    ":memchr",
    "//complicated_cargo_library/vendor/libc-0.2.24:libc",
    
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    
    crate_features = [
      "default",
      "libc",
      "use_std",
    ],
    visibility = ["//visibility:private"],
)
rust_library(
    name = "memchr",
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
      "default",
      "libc",
      "use_std",
    ],
)
