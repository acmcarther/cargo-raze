# hello_cargo_library


## Manual steps run

1. Run `cargo install cargo-vendor`
2. Run `cargo install cargo-raze`
3. Generate a Cargo.toml with desired dependencies
4. Run `cargo generate-lockfile`
5. Run `cargo vendor -x` (the -x forces versioning)
6. Run `cargo raze`

At this point you will have a dependency specification that Bazel can understand. Next, a bit of boilerplate is added as BUILD files for each rule.

```python
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
```

This generates the `rust_library` declarations from the `cargo-raze` invocation's output data.

To expose those dependencies, `alias` entries are created for the explicit Cargo dependencies. It is important to only expose explicit dependencies for the sake of hygiene.

```python
package(default_visibility = ["//visibility:public"])

alias(
    name = "fern",
    actual = "//examples/hello_cargo_library/vendor/fern-0.3.5:fern"
)

alias(
    name = "log",
    actual = "//examples/hello_cargo_library/vendor/log-0.3.7:log"
)
```
