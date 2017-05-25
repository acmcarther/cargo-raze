"""
cargo-raze generated details for cargo-0.17.0.

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
        pkg_name = "cargo",
        pkg_version = "0.17.0",
    ),
    dependencies = [
        struct(
            name = "url",
            version = "1.4.0",
        ),
        struct(
            name = "libgit2-sys",
            version = "0.6.10",
        ),
        struct(
            name = "term",
            version = "0.4.5",
        ),
        struct(
            name = "log",
            version = "0.3.8",
        ),
        struct(
            name = "git2",
            version = "0.6.5",
        ),
        struct(
            name = "docopt",
            version = "0.6.86",
        ),
        struct(
            name = "git2-curl",
            version = "0.7.0",
        ),
        struct(
            name = "fs2",
            version = "0.3.0",
        ),
        struct(
            name = "tar",
            version = "0.4.12",
        ),
        struct(
            name = "glob",
            version = "0.2.11",
        ),
        struct(
            name = "env_logger",
            version = "0.3.5",
        ),
        struct(
            name = "num_cpus",
            version = "1.4.0",
        ),
        struct(
            name = "regex",
            version = "0.1.80",
        ),
        struct(
            name = "tempdir",
            version = "0.3.5",
        ),
        struct(
            name = "openssl",
            version = "0.9.12",
        ),
        struct(
            name = "semver",
            version = "0.5.1",
        ),
        struct(
            name = "flate2",
            version = "0.2.19",
        ),
        struct(
            name = "libc",
            version = "0.2.23",
        ),
        struct(
            name = "toml",
            version = "0.2.1",
        ),
        struct(
            name = "crossbeam",
            version = "0.2.10",
        ),
        struct(
            name = "rustc-serialize",
            version = "0.3.24",
        ),
        struct(
            name = "shell-escape",
            version = "0.1.3",
        ),
        struct(
            name = "filetime",
            version = "0.1.10",
        ),
        struct(
            name = "crates-io",
            version = "0.6.0",
        ),
        struct(
            name = "curl",
            version = "0.4.6",
        ),
    ],
    build_dependencies = [],
    dev_dependencies = [
        struct(
            name = "filetime",
            version = "0.1.10",
        ),
    ],
    features = [],
    targets = [
        struct(
            name = "cargo",
            kinds = [
                "lib",
            ],
            path = "src/cargo/lib.rs",
        ),
        struct(
            name = "cargo",
            kinds = [
                "bin",
            ],
            path = "src/bin/cargo.rs",
        ),
        struct(
            name = "plugins",
            kinds = [
                "test",
            ],
            path = "tests/plugins.rs",
        ),
        struct(
            name = "git",
            kinds = [
                "test",
            ],
            path = "tests/git.rs",
        ),
        struct(
            name = "freshness",
            kinds = [
                "test",
            ],
            path = "tests/freshness.rs",
        ),
        struct(
            name = "cfg",
            kinds = [
                "test",
            ],
            path = "tests/cfg.rs",
        ),
        struct(
            name = "bench",
            kinds = [
                "test",
            ],
            path = "tests/bench.rs",
        ),
        struct(
            name = "build-script",
            kinds = [
                "test",
            ],
            path = "tests/build-script.rs",
        ),
        struct(
            name = "rustdocflags",
            kinds = [
                "test",
            ],
            path = "tests/rustdocflags.rs",
        ),
        struct(
            name = "version",
            kinds = [
                "test",
            ],
            path = "tests/version.rs",
        ),
        struct(
            name = "new",
            kinds = [
                "test",
            ],
            path = "tests/new.rs",
        ),
        struct(
            name = "shell",
            kinds = [
                "test",
            ],
            path = "tests/shell.rs",
        ),
        struct(
            name = "clean",
            kinds = [
                "test",
            ],
            path = "tests/clean.rs",
        ),
        struct(
            name = "profiles",
            kinds = [
                "test",
            ],
            path = "tests/profiles.rs",
        ),
        struct(
            name = "package",
            kinds = [
                "test",
            ],
            path = "tests/package.rs",
        ),
        struct(
            name = "read-manifest",
            kinds = [
                "test",
            ],
            path = "tests/read-manifest.rs",
        ),
        struct(
            name = "death",
            kinds = [
                "test",
            ],
            path = "tests/death.rs",
        ),
        struct(
            name = "proc-macro",
            kinds = [
                "test",
            ],
            path = "tests/proc-macro.rs",
        ),
        struct(
            name = "rustdoc",
            kinds = [
                "test",
            ],
            path = "tests/rustdoc.rs",
        ),
        struct(
            name = "dep-info",
            kinds = [
                "test",
            ],
            path = "tests/dep-info.rs",
        ),
        struct(
            name = "bad-manifest-path",
            kinds = [
                "test",
            ],
            path = "tests/bad-manifest-path.rs",
        ),
        struct(
            name = "net-config",
            kinds = [
                "test",
            ],
            path = "tests/net-config.rs",
        ),
        struct(
            name = "features",
            kinds = [
                "test",
            ],
            path = "tests/features.rs",
        ),
        struct(
            name = "cargo",
            kinds = [
                "test",
            ],
            path = "tests/cargo.rs",
        ),
        struct(
            name = "directory",
            kinds = [
                "test",
            ],
            path = "tests/directory.rs",
        ),
        struct(
            name = "local-registry",
            kinds = [
                "test",
            ],
            path = "tests/local-registry.rs",
        ),
        struct(
            name = "doc",
            kinds = [
                "test",
            ],
            path = "tests/doc.rs",
        ),
        struct(
            name = "verify-project",
            kinds = [
                "test",
            ],
            path = "tests/verify-project.rs",
        ),
        struct(
            name = "rustflags",
            kinds = [
                "test",
            ],
            path = "tests/rustflags.rs",
        ),
        struct(
            name = "publish",
            kinds = [
                "test",
            ],
            path = "tests/publish.rs",
        ),
        struct(
            name = "build-auth",
            kinds = [
                "test",
            ],
            path = "tests/build-auth.rs",
        ),
        struct(
            name = "generate-lockfile",
            kinds = [
                "test",
            ],
            path = "tests/generate-lockfile.rs",
        ),
        struct(
            name = "lockfile-compat",
            kinds = [
                "test",
            ],
            path = "tests/lockfile-compat.rs",
        ),
        struct(
            name = "metadata",
            kinds = [
                "test",
            ],
            path = "tests/metadata.rs",
        ),
        struct(
            name = "cargo_alias_config",
            kinds = [
                "test",
            ],
            path = "tests/cargo_alias_config.rs",
        ),
        struct(
            name = "registry",
            kinds = [
                "test",
            ],
            path = "tests/registry.rs",
        ),
        struct(
            name = "workspaces",
            kinds = [
                "test",
            ],
            path = "tests/workspaces.rs",
        ),
        struct(
            name = "resolve",
            kinds = [
                "test",
            ],
            path = "tests/resolve.rs",
        ),
        struct(
            name = "search",
            kinds = [
                "test",
            ],
            path = "tests/search.rs",
        ),
        struct(
            name = "build",
            kinds = [
                "test",
            ],
            path = "tests/build.rs",
        ),
        struct(
            name = "tool-paths",
            kinds = [
                "test",
            ],
            path = "tests/tool-paths.rs",
        ),
        struct(
            name = "overrides",
            kinds = [
                "test",
            ],
            path = "tests/overrides.rs",
        ),
        struct(
            name = "install",
            kinds = [
                "test",
            ],
            path = "tests/install.rs",
        ),
        struct(
            name = "test",
            kinds = [
                "test",
            ],
            path = "tests/test.rs",
        ),
        struct(
            name = "concurrent",
            kinds = [
                "test",
            ],
            path = "tests/concurrent.rs",
        ),
        struct(
            name = "init",
            kinds = [
                "test",
            ],
            path = "tests/init.rs",
        ),
        struct(
            name = "run",
            kinds = [
                "test",
            ],
            path = "tests/run.rs",
        ),
        struct(
            name = "build-lib",
            kinds = [
                "test",
            ],
            path = "tests/build-lib.rs",
        ),
        struct(
            name = "bad-config",
            kinds = [
                "test",
            ],
            path = "tests/bad-config.rs",
        ),
        struct(
            name = "path",
            kinds = [
                "test",
            ],
            path = "tests/path.rs",
        ),
        struct(
            name = "config",
            kinds = [
                "test",
            ],
            path = "tests/config.rs",
        ),
        struct(
            name = "fetch",
            kinds = [
                "test",
            ],
            path = "tests/fetch.rs",
        ),
        struct(
            name = "cross-compile",
            kinds = [
                "test",
            ],
            path = "tests/cross-compile.rs",
        ),
        struct(
            name = "check",
            kinds = [
                "test",
            ],
            path = "tests/check.rs",
        ),
        struct(
            name = "rustc",
            kinds = [
                "test",
            ],
            path = "tests/rustc.rs",
        ),
    ],
)
