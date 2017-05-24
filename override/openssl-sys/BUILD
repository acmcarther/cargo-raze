package(default_visibility = ["//visibility:public"])

licenses(["notice"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
)

rust_library(
    name = "openssl_sys",
    srcs = [
        "//vendor/openssl-sys-0.9.12:sources"
    ],
    deps = [
        "//vendor/libc-0.2.22:libc",
    ],
    rustc_flags = [
        "--cap-lints warn",
    ],
    crate_features = [
    ],
)
