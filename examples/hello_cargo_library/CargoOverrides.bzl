"""
cargo-raze vendor-wide override file

Make your changes here. Bazel automatically integrates overrides from this
file and will not overwrite it on a rerun of cargo-raze.

Override entries should be of identical form to generated Crate.bzl entries.
Properties defined here will take priority over generated properties.

Reruns of cargo-raze may change the versions of your dependencies. Fear not!
cargo-raze will warn you if it detects an override for different version of a
dependency, to prompt you to update the specified override version.
"""
overrides = []
