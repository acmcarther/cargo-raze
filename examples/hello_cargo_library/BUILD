load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary",)

rust_binary(
    name = "hello",
    srcs = ["src/main.rs"],
    deps = [
        "//hello_cargo_library/vendor:log",
        "//hello_cargo_library/vendor:fern",
    ],
)
