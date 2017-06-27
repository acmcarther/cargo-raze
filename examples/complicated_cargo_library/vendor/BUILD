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
    name = "regex",
    actual = "//examples/complicated_cargo_library/vendor/regex-0.2.2:regex",
)

alias(
    name = "x11",
    actual = "//examples/complicated_cargo_library/vendor/x11-2.14.0:x11",
)

