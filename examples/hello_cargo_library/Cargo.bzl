"""
cargo-raze vendor-wide workspace file

DO NOT EDIT! Replaced on runs of cargo-raze
"""

workspace = struct(
    platform = struct(
        triple = "x86_64-unknown-linux-gnu",
        flags = [
            "debug_assertions",
            "target_thread_local",
            "unix",
        ],
        vars = [
            (
                "target_arch",
                "x86_64",
            ),
            (
                "target_endian",
                "little",
            ),
            (
                "target_env",
                "gnu",
            ),
            (
                "target_family",
                "unix",
            ),
            (
                "target_feature",
                "sse",
            ),
            (
                "target_feature",
                "sse2",
            ),
            (
                "target_has_atomic",
                "16",
            ),
            (
                "target_has_atomic",
                "32",
            ),
            (
                "target_has_atomic",
                "64",
            ),
            (
                "target_has_atomic",
                "8",
            ),
            (
                "target_has_atomic",
                "ptr",
            ),
            (
                "target_os",
                "linux",
            ),
            (
                "target_pointer_width",
                "64",
            ),
            (
                "target_vendor",
                "unknown",
            ),
        ],
    ),
    packages = [
        struct(
            package = struct(
                pkg_name = "fern",
                pkg_version = "0.3.5",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [
                struct(
                    name = "log",
                    version = "0.3.7",
                ),
            ],
            build_dependencies = [],
            dev_dependencies = [],
            features = [],
            targets = [
                struct(
                    name = "doc_test_copy",
                    kinds = [
                        "test",
                    ],
                    path = "tests/doc_test_copy.rs",
                ),
                struct(
                    name = "fern",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
                struct(
                    name = "lib",
                    kinds = [
                        "test",
                    ],
                    path = "tests/lib.rs",
                ),
            ],
        ),
        struct(
            package = struct(
                pkg_name = "log",
                pkg_version = "0.3.7",
            ),
            bazel_config = struct(
                use_build_rs = True,
                use_metadeps = False,
            ),
            metadeps = [],
            dependencies = [],
            build_dependencies = [],
            dev_dependencies = [],
            features = [
                "default",
                "use_std",
            ],
            targets = [
                struct(
                    name = "filters",
                    kinds = [
                        "test",
                    ],
                    path = "tests/filters.rs",
                ),
                struct(
                    name = "log",
                    kinds = [
                        "lib",
                    ],
                    path = "src/lib.rs",
                ),
            ],
        ),
    ],
)
