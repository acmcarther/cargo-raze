# cargo raze
A cargo subcommand to generate platform-specific BUILD files.

## Problem

You like cargo's package rich ecosystem, but are interested in using Bazel to build a multilanguage, large, or otherwise complex set of applications.

So far you've either stuck with Cargo and made do with `build.rs` files, or migrated to Bazel and avoided `crates.io` dependencies or manually generated a select set of BUILD files for vendored dependencies.

`cargo raze` gives you the best of both worlds: rust library downloading + resolution courtesy of Cargo with the power and scalability of Bazel.

## How it looks (speculative and untested)

In a directory containing 'Cargo.toml'
```
cargo install cargo-vendor
cargo install cargo-raze
cargo generate_lockfile
cargo vendor -x
cargo raze
```
You dependencies each get a shiny new `Cargo.bzl` file that bazel can use to link your dependencies up

## How it works (soon!)

`cargo raze` uses Cargo's own internal dependency resolution, feature flag propagation, and platform introspection to link the vendored dependencies properly for your platform.

## TODO:

- The bazel half of this rule. Even though we generate handy `bzl` files, nobody's on the other end of the line to receive our configuration.
- Proper platform detection. Currently we just use generic linux. This isn't too hard, just haven't had time.
- Platform-agnostic generated `Cargo.bzl`. I envision mapping the existing platform-specific dependency support down to a handful of supported platforms within the bazel rule, rather than here. That lets us use bazel's `select` construct to support multiple platforms with a single rule.
