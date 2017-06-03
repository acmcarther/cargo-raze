# cargo raze

A cargo subcommand to generate platform-specific BUILD files.

Also, a bazel ruleset for using the outputs of that cargo subcommand.

## Problem

You like cargo's package rich ecosystem, but are interested in using Bazel to build a multilanguage, large, or otherwise complex set of applications.

So far you've either stuck with Cargo and made do with `build.rs` files, or migrated to Bazel and avoided `crates.io` dependencies or manually generated a select set of BUILD files for vendored dependencies.

`cargo raze` gives you the best of both worlds: rust library downloading + resolution courtesy of Cargo with the power and scalability of Bazel.

## Getting Started

In your Bazel WORKSPACE:
```python
git_repository(
    name = "io_bazel_rules_raze",
    remote = "https://github.com/acmcarther/cargo-raze.git",
    commit = "c84e361"
)

git_repository(
    name = "io_bazel_rules_rust",
    remote = "https://github.com/acmcarther/rules_rust.git",
    commit = "49a7345"
)
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_repositories")

rust_repositories()
```

Then, in a directory containing 'Cargo.toml'. Project root is fine:
```
cargo install cargo-vendor
cargo install cargo-raze
cargo generate_lockfile
cargo vendor -x
cargo raze "//path/to/vendor"
```
You dependencies each get a shiny new `Cargo.bzl` file that bazel can use to link your dependencies up. You will also get starter BUILD files that reference those .bzl files.

See the [example](examples/hello_cargo_library/README.md) for further details.

See my hobby project [space_coop](https://github.com/acmcarther/next_space_coop) for a real life example.

## Project Structure

This repo is a hybrid cargo crate + Bazel Skylark ruleset. The project is structured roughly as follows:

cargo-raze crate:
- ./Cargo.toml
- ./Cargo.lock
- ./src/

rules_raze:
- ./WORKSPACE
- ./examples/
- ./raze/


## TODO:

- Proper platform detection. Currently we just use generic linux. This isn't too hard, just haven't had time.
- Platform-agnostic generated `Cargo.bzl`. I envision mapping the existing platform-specific dependency support down to a handful of supported platforms within the bazel rule, rather than here. That lets us use bazel's `select` construct to support multiple platforms with a single rule.
- Clean up folder structure
- Set up dual compilation (compile raze via bazel OR via cargo). Prereq: openssl. Openssl is major pain to compile.
