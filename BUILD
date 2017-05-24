'''
WARNING: THIS IS GENERATED CODE!
DO NOT MODIFY!
Instead, rerun raze with different arguments.
'''
package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "raze",
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    deps = [
        "//vendor/cargo-0.17.0:cargo",
        "//vendor/nom-2.2.1:nom",
        "//vendor/rustc-serialize-0.3.24:rustc_serialize",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
