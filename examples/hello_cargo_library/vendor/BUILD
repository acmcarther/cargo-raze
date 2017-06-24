"""
cargo-raze direct Cargo.toml dependencies.

This BUILD file provides aliases to explicit cargo dependencies and is
the only way to access vendored dependencies.
If a dependency is missing, add it as an explicit root dependency in
Cargo.toml and rerun raze.
This file is overridden on runs of raze; do not add anything to it.
If that is causing you pain, please drop a line in acmcarther/cargo-raze.
"""
package(default_visibility = ["//visibility:public"])

alias(
    name = "fern",
    actual = "//examples/hello_cargo_library/vendor/fern-0.3.5:fern",
)

alias(
    name = "log",
    actual = "//examples/hello_cargo_library/vendor/log-0.3.7:log",
)

