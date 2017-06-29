"""
cargo-raze generated details for x11-2.14.0.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "x11",
        pkg_version = "2.14.0",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    metadeps = [],
    dependencies = [
        struct(
            name = "libc",
            version = "0.2.24",
        ),
    ],
    build_dependencies = [
        struct(
            name = "metadeps",
            version = "1.1.2",
        ),
    ],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "build-script-build",
            kinds = [
                "custom-build",
            ],
            path = "build.rs",
        ),
        struct(
            name = "hello-world",
            kinds = [
                "example",
            ],
            path = "examples/hello-world.rs",
        ),
        struct(
            name = "input",
            kinds = [
                "example",
            ],
            path = "examples/input.rs",
        ),
        struct(
            name = "x11",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "xrecord",
            kinds = [
                "example",
            ],
            path = "examples/xrecord.rs",
        ),
    ],
)
