extern crate cargotest;
extern crate hamcrest;

use hamcrest::assert_that;
use cargotest::support::registry::Package;
use cargotest::support::{project, execs, basic_bin_manifest, basic_lib_manifest, main_file};

#[test]
fn cargo_metadata_simple() {
    let p = project("foo")
            .file("Cargo.toml", &basic_bin_manifest("foo"));

    assert_that(p.cargo_process("metadata"), execs().with_json(r#"
    {
        "packages": [
            {
                "name": "foo",
                "version": "0.5.0",
                "id": "foo[..]",
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "targets": [
                    {
                        "kind": [
                            "bin"
                        ],
                        "name": "foo",
                        "src_path": "[..][/]foo[/]src[/]foo.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]Cargo.toml"
            }
        ],
        "workspace_members": ["foo 0.5.0 (path+file:[..]foo)"],
        "resolve": {
            "nodes": [
                {
                    "dependencies": [],
                    "id": "foo 0.5.0 (path+file:[..]foo)"
                }
            ],
            "root": "foo 0.5.0 (path+file:[..]foo)"
        },
        "version": 1
    }"#));
}


#[test]
fn cargo_metadata_with_deps_and_version() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.5.0"
            authors = []
            license = "MIT"
            description = "foo"

            [[bin]]
            name = "foo"

            [dependencies]
            bar = "*"
        "#);
    Package::new("baz", "0.0.1").publish();
    Package::new("bar", "0.0.1").dep("baz", "0.0.1").publish();

    assert_that(p.cargo_process("metadata")
                 .arg("-q")
                 .arg("--format-version").arg("1"),
                execs().with_json(r#"
    {
        "packages": [
            {
                "dependencies": [],
                "features": {},
                "id": "baz 0.0.1 (registry+[..])",
                "manifest_path": "[..]Cargo.toml",
                "name": "baz",
                "source": "registry+[..]",
                "license": null,
                "license_file": null,
                "targets": [
                    {
                        "kind": [
                            "lib"
                        ],
                        "name": "baz",
                        "src_path": "[..]lib.rs"
                    }
                ],
                "version": "0.0.1"
            },
            {
                "dependencies": [
                    {
                        "features": [],
                        "kind": null,
                        "name": "baz",
                        "optional": false,
                        "req": "^0.0.1",
                        "source": "registry+[..]",
                        "target": null,
                        "uses_default_features": true
                    }
                ],
                "features": {},
                "id": "bar 0.0.1 (registry+[..])",
                "manifest_path": "[..]Cargo.toml",
                "name": "bar",
                "source": "registry+[..]",
                "license": null,
                "license_file": null,
                "targets": [
                    {
                        "kind": [
                            "lib"
                        ],
                        "name": "bar",
                        "src_path": "[..]lib.rs"
                    }
                ],
                "version": "0.0.1"
            },
            {
                "dependencies": [
                    {
                        "features": [],
                        "kind": null,
                        "name": "bar",
                        "optional": false,
                        "req": "*",
                        "source": "registry+[..]",
                        "target": null,
                        "uses_default_features": true
                    }
                ],
                "features": {},
                "id": "foo 0.5.0 (path+file:[..]foo)",
                "manifest_path": "[..]Cargo.toml",
                "name": "foo",
                "source": null,
                "license": "MIT",
                "license_file": null,
                "targets": [
                    {
                        "kind": [
                            "bin"
                        ],
                        "name": "foo",
                        "src_path": "[..]foo.rs"
                    }
                ],
                "version": "0.5.0"
            }
        ],
        "workspace_members": ["foo 0.5.0 (path+file:[..]foo)"],
        "resolve": {
            "nodes": [
                {
                    "dependencies": [
                        "bar 0.0.1 (registry+[..])"
                    ],
                    "id": "foo 0.5.0 (path+file:[..]foo)"
                },
                {
                    "dependencies": [
                        "baz 0.0.1 (registry+[..])"
                    ],
                    "id": "bar 0.0.1 (registry+[..])"
                },
                {
                    "dependencies": [],
                    "id": "baz 0.0.1 (registry+[..])"
                }
            ],
            "root": "foo 0.5.0 (path+file:[..]foo)"
        },
        "version": 1
    }"#));
}

#[test]
fn workspace_metadata() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
            members = ["bar", "baz"]
        "#)
        .file("bar/Cargo.toml", &basic_lib_manifest("bar"))
        .file("bar/src/lib.rs", "")
        .file("baz/Cargo.toml", &basic_lib_manifest("baz"))
        .file("baz/src/lib.rs", "");
    p.build();

    assert_that(p.cargo_process("metadata"), execs().with_status(0).with_json(r#"
    {
        "packages": [
            {
                "name": "bar",
                "version": "0.5.0",
                "id": "bar[..]",
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "targets": [
                    {
                        "kind": [ "lib" ],
                        "name": "bar",
                        "src_path": "[..]bar[/]src[/]lib.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]bar[/]Cargo.toml"
            },
            {
                "name": "baz",
                "version": "0.5.0",
                "id": "baz[..]",
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "targets": [
                    {
                        "kind": [ "lib" ],
                        "name": "baz",
                        "src_path": "[..]baz[/]src[/]lib.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]baz[/]Cargo.toml"
            }
        ],
        "workspace_members": ["baz 0.5.0 (path+file:[..]baz)", "bar 0.5.0 (path+file:[..]bar)"],
        "resolve": {
            "nodes": [
                {
                    "dependencies": [],
                    "id": "baz 0.5.0 (path+file:[..]baz)"
                },
                {
                    "dependencies": [],
                    "id": "bar 0.5.0 (path+file:[..]bar)"
                }
            ],
            "root": null
        },
        "version": 1
    }"#))
}

#[test]
fn workspace_metadata_no_deps() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
            members = ["bar", "baz"]
        "#)
        .file("bar/Cargo.toml", &basic_lib_manifest("bar"))
        .file("bar/src/lib.rs", "")
        .file("baz/Cargo.toml", &basic_lib_manifest("baz"))
        .file("baz/src/lib.rs", "");
    p.build();

    assert_that(p.cargo_process("metadata").arg("--no-deps"), execs().with_status(0).with_json(r#"
    {
        "packages": [
            {
                "name": "bar",
                "version": "0.5.0",
                "id": "bar[..]",
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "targets": [
                    {
                        "kind": [ "lib" ],
                        "name": "bar",
                        "src_path": "[..]bar[/]src[/]lib.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]bar[/]Cargo.toml"
            },
            {
                "name": "baz",
                "version": "0.5.0",
                "id": "baz[..]",
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "targets": [
                    {
                        "kind": [ "lib" ],
                        "name": "baz",
                        "src_path": "[..]baz[/]src[/]lib.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]baz[/]Cargo.toml"
            }
        ],
        "workspace_members": ["baz 0.5.0 (path+file:[..]baz)", "bar 0.5.0 (path+file:[..]bar)"],
        "resolve": null,
        "version": 1
    }"#))
}

#[test]
fn cargo_metadata_with_invalid_manifest() {
    let p = project("foo")
            .file("Cargo.toml", "");

    assert_that(p.cargo_process("metadata"), execs().with_status(101)
                                                    .with_stderr("\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  no `package` or `project` section found."))
}

const MANIFEST_OUTPUT: &'static str=
    r#"
{
    "packages": [{
        "name":"foo",
        "version":"0.5.0",
        "id":"foo[..]0.5.0[..](path+file://[..]/foo)",
        "source":null,
        "dependencies":[],
        "license": null,
        "license_file": null,
        "targets":[{
            "kind":["bin"],
            "name":"foo",
            "src_path":"[..][/]foo[/]src[/]foo.rs"
        }],
        "features":{},
        "manifest_path":"[..]Cargo.toml"
    }],
    "workspace_members": [ "foo 0.5.0 (path+file:[..]foo)" ],
    "resolve": null,
    "version": 1
}"#;

#[test]
fn cargo_metadata_no_deps_path_to_cargo_toml_relative() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

        assert_that(p.cargo_process("metadata").arg("--no-deps")
                     .arg("--manifest-path").arg("foo/Cargo.toml")
                     .cwd(p.root().parent().unwrap()),
                    execs().with_status(0)
                           .with_json(MANIFEST_OUTPUT));
}

#[test]
fn cargo_metadata_no_deps_path_to_cargo_toml_absolute() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.cargo_process("metadata").arg("--no-deps")
                 .arg("--manifest-path").arg(p.root().join("Cargo.toml"))
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(0)
                       .with_json(MANIFEST_OUTPUT));
}

#[test]
fn cargo_metadata_no_deps_path_to_cargo_toml_parent_relative() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.cargo_process("metadata").arg("--no-deps")
                 .arg("--manifest-path").arg("foo")
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(101)
                       .with_stderr("[ERROR] the manifest-path must be \
                                             a path to a Cargo.toml file"));
}

#[test]
fn cargo_metadata_no_deps_path_to_cargo_toml_parent_absolute() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.cargo_process("metadata").arg("--no-deps")
                 .arg("--manifest-path").arg(p.root())
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(101)
                       .with_stderr("[ERROR] the manifest-path must be \
                                             a path to a Cargo.toml file"));
}

#[test]
fn cargo_metadata_no_deps_cwd() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.cargo_process("metadata").arg("--no-deps")
                 .cwd(p.root()),
                execs().with_status(0)
                       .with_json(MANIFEST_OUTPUT));
}

#[test]
fn carg_metadata_bad_version() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.cargo_process("metadata").arg("--no-deps")
                 .arg("--format-version").arg("2")
                 .cwd(p.root()),
                execs().with_status(101)
    .with_stderr("[ERROR] metadata version 2 not supported, only 1 is currently supported"));
}

#[test]
fn multiple_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []

            [features]
            a = []
            b = []
        "#)
        .file("src/lib.rs", "");

    assert_that(p.cargo_process("metadata")
                 .arg("--features").arg("a b"),
                execs().with_status(0));
}
