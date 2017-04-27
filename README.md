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
cargo vendor ./ -x
cargo raze
```
You dependencies appear in the local directory complete with BUILD files and a WORKSPACE.

## But that sounds like a pain

See [bazel raze](https://github.com/acmcarther/bazel-raze)

## How it works (soon!)

Using `cargo-vendor` and `cargo-raze`, Bazel performs Cargo dependency vendoring, and then supplements the dependencies with BUILD files. Either step can be perfomed manually, thereby "locking" either the sources or the BUILD rules for the whole project.

One important note: BUILD files are specific to the platform that `cargo-raze` is executed on. If you wish continue to leverage Cargo's platform-specific dependency resolution, you will need to defer generation of the BUILD files.
