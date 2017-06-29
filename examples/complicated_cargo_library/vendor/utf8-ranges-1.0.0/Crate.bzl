"""
cargo-raze generated details for utf8-ranges-1.0.0.

DO NOT MODIFY! Instead, update CargoOverrides.bzl.
"""
description = struct(
    package = struct(
        pkg_name = "utf8-ranges",
        pkg_version = "1.0.0",
    ),
    bazel_config = struct(
        use_build_rs = True,
        use_metadeps = False,
    ),
    metadeps = [],
    dependencies = [],
    build_dependencies = [],
    dev_dependencies = [],
    features = [],
    targets = [
        struct(
            name = "bench",
            kinds = [
                "bench",
            ],
            path = "benches/bench.rs",
        ),
        struct(
            name = "utf8-ranges",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
    ],
)
