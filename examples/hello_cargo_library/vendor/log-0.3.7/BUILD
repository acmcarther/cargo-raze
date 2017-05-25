package(default_visibility = ["//examples/hello_cargo_library/vendor:__subpackages__"])

load("//raze:raze.bzl", "cargo_library")
load(":Cargo.bzl", "description")
load(":CargoOverride.bzl", "override")

cargo_library(
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    cargo_bzl = description,
    cargo_override_bzl = override,
    workspace_path = "//examples/hello_cargo_library/vendor/"
)

