"""
cargo-raze generated details for git2-0.6.5.

Generated for:
platform_triple: x86_64-unknown-linux-gnu
platform_attrs:
[
    "debug_assertions",
    "target_arch: x86_64",
    "target_endian: little",
    "target_env: gnu",
    "target_family: unix",
    "target_feature: sse",
    "target_feature: sse2",
    "target_has_atomic: 16",
    "target_has_atomic: 32",
    "target_has_atomic: 64",
    "target_has_atomic: 8",
    "target_has_atomic: ptr",
    "target_os: linux",
    "target_pointer_width: 64",
    "target_thread_local",
    "target_vendor: unknown",
    "unix"
]

DO NOT MODIFY! Instead, add a CargoOverride.bzl mixin.
"""
description = struct(
    package = struct(
        pkg_name = "git2",
        pkg_version = "0.6.5",
    ),
    dependencies = [
        struct(
            name = "url",
            version = "1.4.0",
        ),
        struct(
            name = "bitflags",
            version = "0.7.0",
        ),
        struct(
            name = "openssl-probe",
            version = "0.1.1",
        ),
        struct(
            name = "openssl-sys",
            version = "0.9.12",
        ),
        struct(
            name = "libc",
            version = "0.2.23",
        ),
        struct(
            name = "libgit2-sys",
            version = "0.6.10",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [],
    features = [
        "openssl-probe",
        "https",
        "ssh",
        "default",
        "curl",
        "libgit2-sys",
        "openssl-sys",
    ],
    targets = [
        struct(
            name = "git2",
            kinds = [
                "lib",
            ],
            path = "src/lib.rs",
        ),
        struct(
            name = "ls-remote",
            kinds = [
                "example",
            ],
            path = "examples/ls-remote.rs",
        ),
        struct(
            name = "rev-parse",
            kinds = [
                "example",
            ],
            path = "examples/rev-parse.rs",
        ),
        struct(
            name = "log",
            kinds = [
                "example",
            ],
            path = "examples/log.rs",
        ),
        struct(
            name = "add",
            kinds = [
                "example",
            ],
            path = "examples/add.rs",
        ),
        struct(
            name = "tag",
            kinds = [
                "example",
            ],
            path = "examples/tag.rs",
        ),
        struct(
            name = "blame",
            kinds = [
                "example",
            ],
            path = "examples/blame.rs",
        ),
        struct(
            name = "status",
            kinds = [
                "example",
            ],
            path = "examples/status.rs",
        ),
        struct(
            name = "cat-file",
            kinds = [
                "example",
            ],
            path = "examples/cat-file.rs",
        ),
        struct(
            name = "rev-list",
            kinds = [
                "example",
            ],
            path = "examples/rev-list.rs",
        ),
        struct(
            name = "init",
            kinds = [
                "example",
            ],
            path = "examples/init.rs",
        ),
        struct(
            name = "clone",
            kinds = [
                "example",
            ],
            path = "examples/clone.rs",
        ),
        struct(
            name = "diff",
            kinds = [
                "example",
            ],
            path = "examples/diff.rs",
        ),
        struct(
            name = "fetch",
            kinds = [
                "example",
            ],
            path = "examples/fetch.rs",
        ),
    ],
)
