workspace(name = "io_bazel_rules_raze")

local_repository(
    name = "examples",
    path = "examples",
)

# For examples/ dir
git_repository(
    name = "io_bazel_rules_rust",
    remote = "https://github.com/acmcarther/rules_rust.git",
    commit = "c9b9c1a"
)
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

# For examples/ dir
rust_repositories()
