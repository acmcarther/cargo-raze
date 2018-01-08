# cargo raze

[![Build Status](https://travis-ci.org/acmcarther/cargo-raze.svg?branch=master)](https://travis-ci.org/acmcarther/cargo-raze)

A cargo subcommand to generate platform-specific BUILD files.

See examples in the automatically updated examples repository:
[github.com/acmcarther/cargo-raze-examples](https://github.com/acmcarther/cargo-raze-examples)

## Caution: Under Development!

This is still under heavy development. It has few tests and is very unstable. It relies on some custom changes to the [rules_rust](https://github.com/bazel/rules_rust) library to support build scripts and duplicate crate definitions. The changes are available in the [diff between my repo and master](https://github.com/bazelbuild/rules_rust/compare/master...acmcarther:acm-06-17-hotfixes).

If you'd like to use it anyway, you are definitely welcome to. Please direct any questions to acmcarther@: your input would be very helpful to guide development

## Problem

You like cargo's package rich ecosystem, but are interested in using Bazel to build a multilanguage, large, or otherwise complex set of applications.

So far you've either stuck with Cargo and made do with `build.rs` files, or migrated to Bazel and avoided `crates.io` dependencies or manually generated a select set of BUILD files for vendored dependencies.

`cargo raze` gives you the best of both worlds: rust library downloading + resolution courtesy of Cargo with the power and scalability of Bazel.

## Getting Started

In your Bazel WORKSPACE, include the rust rules:
```python
git_repository(
    name = "io_bazel_rules_rust",
    commit = "5b94fdb",
    remote = "https://github.com/acmcarther/rules_rust.git",
)
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()
```

Create a `Cargo.toml` containing the crates you want to include and configuration for raze:
```toml
[package]
name = "compile_with_bazel"
version = "0.1.0"

[dependencies]
log = "0.3.6"

[raze]
workspace_path = "//label/for/vendored/crates" # The WORKSPACE relative path to the Cargo.toml working directory. 
target = "x86_64-unknown-linux-gnu" # The target to generate BUILD rules for.
```

Then, in the directory containing the `Cargo.toml`. Project root is fine:
```bash
cargo install cargo-vendor
cargo install cargo-raze
cargo generate-lockfile
cargo vendor -x
cargo raze
```
Your dependencies each get a shiny new `BUILD` file that bazel can use to link your dependencies up.

See my hobby project [space_coop](https://github.com/acmcarther/next_space_coop) for a real life example, with its raze Cargo.toml [here](https://github.com/acmcarther/next_space_coop/blob/master/cargo/Cargo.toml).

## Additional Configuration

Sometimes it's necessary to change the way a crate is built, generally to provide a native library or provide configuration.

See these examples of providing crate configuration:

- [basic-example](https://github.com/acmcarther/cargo-raze-examples/blob/master/bazel/hello_cargo_library/Cargo.toml)
- [complicated-example](https://github.com/acmcarther/cargo-raze-examples/blob/master/bazel/complicated_cargo_library/Cargo.toml)
- [openssl-example](https://github.com/acmcarther/compile_openssl/blob/master/cargo/Cargo.toml)

## TODO:

- Proper platform detection. Currently we take platform as a configuration parameter. This isn't too hard, just haven't had time.
- Platform-agnostic generated `BUILD`. I envision mapping the existing platform-specific dependency support down to a handful of supported platforms within the bazel rule, rather than here. That lets us use bazel's `select` construct to support multiple platforms with a single rule.
- Clean up folder structure
- Set up dual compilation (compile raze via bazel OR via cargo). Prereq: openssl. Openssl is major pain to compile.
