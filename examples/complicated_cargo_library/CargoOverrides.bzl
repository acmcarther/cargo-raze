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
    dependency_overrides = [
        struct(
            pkg_name = "bazel_mock_bar",
            pkg_version = "8.8.8",
            target_replacement = None,
            source_replacement = "//any_path/bar:bar_sources",
            config_replacement = None,
        ),
        struct(
            pkg_name = "bazel_mock_baz",
            pkg_version = "8.8.8",
            target_replacement = None,
            source_replacement = None,
            config_replacement = struct(
                package = struct(
                    pkg_name = "bazel_mock_baz",
                    pkg_version = "8.8.88",
                ),
                bazel_config = struct(
                    use_build_rs = False,
                    use_metadeps = False,
                ),
                metadeps = [],
                dependencies = [],
                build_dependencies = [],
                dev_dependencies = [],
                features = [],
                targets = [],
            ),
        ),
        struct(
            pkg_name = "bazel_mock_foo",
            pkg_version = "8.8.8",
            target_replacement = "//any_path/foo:separate_foo_target",
            source_replacement = None,
            config_replacement = None,
        ),
    ],
)
