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
You dependencies each get a shiny new `Cargo.bzl` file that bazel can use to link your dependencies up.

Next, visit [bazel-raze](https://github.com/acmcarther/bazel-raze) to see how to include these dependencies in your project.

## How it works (soon!)

`cargo raze` uses Cargo's own internal dependency resolution, feature flag propagation, and platform introspection to link the vendored dependencies properly for your platform.

## An example generated file
```python
"""
cargo-raze generated details for net2-0.2.29.

Generated for:
platform_triple: x86_64-unknown-linux-gnu
platform_attrs:
[
    "debug_assertions",
    "target_arch: x86_64",
    "target_endian: little",
    "target_env: gnu",
    "target_family: unix",
    "target_feature: sse",
    "target_feature: sse2",
    "target_has_atomic: 16",
    "target_has_atomic: 32",
    "target_has_atomic: 64",
    "target_has_atomic: 8",
    "target_has_atomic: ptr",
    "target_os: linux",
    "target_pointer_width: 64",
    "target_thread_local",
    "target_vendor: unknown",
    "unix"
]

DO NOT MODIFY! Instead, add a CargoOverride.bzl mixin.
"""
description = struct(
    package = struct(
        pkg_name = "net2",
        pkg_version = "0.2.29",
    ),
    dependencies = [
        struct(
            name = "cfg-if",
            version = "0.1.0",
        ),
        struct(
            name = "libc",
            version = "0.2.22",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "net2",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "all",
            kinds = [
                "test",
            ],
            path = "tests/all.rs",
        ),
    ],
)
```

## TODO:

- Proper platform detection. Currently we just use generic linux. This isn't too hard, just haven't had time.
- Platform-agnostic generated `Cargo.bzl`. I envision mapping the existing platform-specific dependency support down to a handful of supported platforms within the bazel rule, rather than here. That lets us use bazel's `select` construct to support multiple platforms with a single rule.
