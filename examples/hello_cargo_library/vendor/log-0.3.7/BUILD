"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//hello_cargo_library:__subpackages__"])

load("//hello_cargo_library:Cargo.bzl", "workspace")
load("//hello_cargo_library:CargoOverrides.bzl", "override_cfg")
load(":Crate.bzl", "description")
load("@io_bazel_rules_raze//raze:raze.bzl", "cargo_library")

cargo_library(
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    crate_bzl = description,
    cargo_override_bzl = override_cfg,
    platform = workspace.platform,
    workspace_path = "//hello_cargo_library/"
)

cargo_library(
    srcs = glob(["lib.rs", "src/**/*.rs"]),
    crate_bzl = description,
    cargo_override_bzl = override_cfg,
    platform = workspace.platform,
    workspace_path = "//hello_cargo_library/"
)