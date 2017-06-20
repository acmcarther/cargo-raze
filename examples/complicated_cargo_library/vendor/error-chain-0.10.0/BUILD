package(default_visibility = ["//examples/complicated_cargo_library/vendor:__subpackages__"])

load("@io_bazel_rules_raze//raze:raze.bzl", "cargo_library")
load(":Cargo.bzl", "description")
load("//examples/complicated_cargo_library/vendor:CargoOverrides.bzl", "overrides")

cargo_library(
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    cargo_bzl = description,
    cargo_override_bzl = overrides,
    workspace_path = "//examples/complicated_cargo_library/vendor/"
)
