"""
cargo-raze vendor-wide override file

Make your changes here. Bazel automatically integrates overrides from this
file and will not overwrite it on a rerun of cargo-raze.

Properties defined here will take priority over generated properties.

Reruns of cargo-raze may change the versions of your dependencies. Fear not!
cargo-raze will warn you if it detects an override for different version of a
dependency, to prompt you to update the specified override version.
"""
override_cfg = struct(
    internal_override_file_version = "1",
    global_settings = struct(
        dependency_replacements = [
            struct(
                pkg_name = "regex",
                pkg_version = "8.8.8",
                target = "//foo/bar:baz",
            ),
        ],
    ),
)
