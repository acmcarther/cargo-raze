workspace(name = "io_bazel_rules_raze")

# For examples/ dir
git_repository(
    name = "io_bazel_rules_rust",
    remote = "https://github.com/acmcarther/rules_rust.git",
    commit = "16f38b29"
)
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

# For examples/ dir
rust_repositories()
