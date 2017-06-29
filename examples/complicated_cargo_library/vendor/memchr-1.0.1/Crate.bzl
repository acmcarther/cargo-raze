"""
cargo-raze generated details for memchr-1.0.1.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "memchr",
        pkg_version = "1.0.1",
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
    build_dependencies = [],
    dev_dependencies = [],
    features = [
        "default",
        "libc",
        "use_std",
    ],
    targets = [
        struct(
            name = "bench",
            kinds = [
                "bench",
            ],
            path = "benches/bench.rs",
        ),
        struct(
            name = "memchr",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)
