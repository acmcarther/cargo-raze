# cargo raze

[![Build Status](https://travis-ci.org/acmcarther/cargo-raze.svg?branch=master)](https://travis-ci.org/acmcarther/cargo-raze)

A cargo subcommand to generate platform-specific BUILD files.

See examples in the automatically updated examples repository:
[github.com/acmcarther/cargo-raze-examples](https://github.com/acmcarther/cargo-raze-examples)

Furthermore, see the crate bonanza in the "crater" repo, which brings in many
of the most popular crates. This repo has some of the Raze settings that will be
necessary to build some crates:
[github.com/acmcarther/cargo-raze-crater Cargo.toml](https://github.com/acmcarther/cargo-raze-crater/blob/master/cargo/Cargo.toml)

## Caution: Under Development!

This is still under heavy development. It has few tests and is very unstable. It relies on some custom changes to the [rules_rust](https://github.com/bazel/rules_rust) library to support build scripts and duplicate crate definitions. The changes are available in the [diff between my repo and master](https://github.com/bazelbuild/rules_rust/compare/master...acmcarther:acm-06-17-hotfixes).

If you'd like to use it anyway, you are definitely welcome to. Please direct any questions to acmcarther@: your input would be very helpful to guide development

## Problem

You like cargo's package rich ecosystem, but are interested in using Bazel to build a multilanguage, large, or otherwise complex set of applications.

So far you've either stuck with Cargo and made do with `build.rs` files, or migrated to Bazel and avoided `crates.io` dependencies or manually generated a select set of BUILD files for vendored dependencies.

`cargo raze` gives you the best of both worlds: rust library downloading + resolution courtesy of Cargo with the power and scalability of Bazel.

## Getting Started

To use Raze, you'll need to:

1. Set up your WORKSPACE to contain Rust rules
2. If Vendoring:
    a. Set up your cargo/ directory for vendoring
    b. Install cargo-vendor, and cargo-raze
    c. Generate your lock file
    d. Vendor your dependencies
    e. Generate your BUILD files into the vendored directories
3. If not vendoring
    a. Set up your cargo/ directory for remote dependencies
    b. Install cargo-raze
    c. Generate your BUILD files
    d. Add the crate fetching snippet into your WORKSPACE file.

See below for individual step details


### Set up your WORKSPACE to contain Rust rules

In your Bazel WORKSPACE, include the rust rules:
```python
git_repository(
    name = "io_bazel_rules_rust",
    commit = "5bc46ddca8817072cdae1961b3f9830a2bc3afa7",
    remote = "https://github.com/acmcarther/rules_rust.git",
)
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()
```

Now, choose between vendoring (checking dependencies into repo) or not vendoring

### Vendoring dependencies locally (default)

#### Set up your cargo/ directory for vendoring
Create a `Cargo.toml` containing the crates you want to include and configuration for raze:
```toml
[package]
name = "compile_with_bazel"
version = "0.1.0"

[lib]
path = "fake_lib.rs"

[dependencies]
log = "0.3.6"

[raze]
workspace_path = "//label/for/vendored/crates" # The WORKSPACE relative path to the Cargo.toml working directory. 
target = "x86_64-unknown-linux-gnu" # The target to generate BUILD rules for.
```

#### Install cargo-vendor and cargo-raze
Then, in the directory containing the `Cargo.toml`. Project root is fine:
```bash
cargo install cargo-vendor
cargo install cargo-raze
```

#### Generate your lock file
```bash
cargo generate-lockfile
```

#### Vendor your dependencies
```bash
cargo vendor -x
```

#### Generate your BUILD files into the vendored directories
```
cargo raze
```
Your dependencies each get a shiny new `BUILD` file that bazel can use to link your dependencies up.

### Remote dependencies

#### Set up your cargo/ directory for vendoring
Create a `Cargo.toml` containing the crates you want to include and configuration for raze:
```toml
[package]
name = "compile_with_bazel"
version = "0.1.0"

[lib]
path = "fake_lib.rs"

[dependencies]
log = "0.3.6"

[raze]
workspace_path = "//label/for/vendored/crates" # The WORKSPACE relative path to the Cargo.toml working directory. 
target = "x86_64-unknown-linux-gnu" # The target to generate BUILD rules for.
genmode = "Remote" # Have Bazel pull the dependencies down
```

#### Install cargo-raze
Then, in the directory containing the `Cargo.toml`. Project root is fine:
```bash
cargo install cargo-raze
```

#### Generate your BUILD files
```bash
cargo raze
```
You now have three files: A "crates.bzl" file with a repository function to pull
down your deps, a BUILD file that references your explicit dependencies, and a
set of dep build files for each of your dependencies.


#### Add the crate fetching snippet into your WORKSPACE file
In order for Bazel to know about the dependencies, you need to execute the
repository function provided in "crates.bzl". To do that, add a snippet as
follows to your root WORKSPACE file.

```python
load("//label/for/vendored/crates:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()
```

## Additional Configuration

Sometimes it's necessary to change the way a crate is built, generally to provide a native library or provide configuration.

See these examples of providing crate configuration:

- [basic-example](https://github.com/acmcarther/cargo-raze-examples/blob/master/bazel/hello_cargo_library/Cargo.toml)
- [complicated-example](https://github.com/acmcarther/cargo-raze-examples/blob/master/bazel/complicated_cargo_library/Cargo.toml)
- [complicated-example-remote](https://github.com/acmcarther/cargo-raze-examples/blob/master/bazel/complicated_cargo_library_remote/Cargo.toml)
- [openssl-example](https://github.com/acmcarther/compile_openssl/blob/master/cargo/Cargo.toml)

The [raze] section is derived from a struct declared in [src/settings.rs](./src/settings.rs).

## TODO:

- Proper platform detection. Currently we take platform as a configuration parameter. This isn't too hard, just haven't had time.
- Platform-agnostic generated `BUILD`. I envision mapping the existing platform-specific dependency support down to a handful of supported platforms within the bazel rule, rather than here. That lets us use bazel's `select` construct to support multiple platforms with a single rule.
- Clean up folder structure
- Set up dual compilation (compile raze via bazel OR via cargo).
