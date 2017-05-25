# hello_cargo_library


## Manual steps run

1. Run `cargo install cargo-vendor`
2. Run `cargo install cargo-raze`
3. Generate a Cargo.toml with desired dependencies
4. Run `cargo generate-lockfile`
5. Run `cargo vendor -x` (the -x forces versioning)
6. Run `cargo raze //examples/hello_cargo_library/vendor`

At this point you will have a dependency specification that Bazel can understand. You will also have starter BUILD files that referene the specified dependencies and generate rust_library rules.

To expose those dependencies, `alias` entries are created for the explicit Cargo dependencies. It is important to only expose explicit dependencies for the sake of hygiene. This is not currently done automatically -- so remember to do this.

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
