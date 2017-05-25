#[macro_use]
extern crate cargotest;
extern crate hamcrest;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::str;

use rustc_serialize::json;

use cargotest::support::{project, execs, ProjectBuilder};
use cargotest::support::paths;
use cargotest::support::registry::{Package, cksum};
use hamcrest::assert_that;

fn setup() {
    let root = paths::root();
    t!(fs::create_dir(&root.join(".cargo")));
    t!(t!(File::create(root.join(".cargo/config"))).write_all(br#"
        [source.crates-io]
        registry = 'https://wut'
        replace-with = 'my-awesome-local-registry'

        [source.my-awesome-local-registry]
        directory = 'index'
    "#));
}

struct VendorPackage {
    p: Option<ProjectBuilder>,
    cksum: Checksum,
}

#[derive(RustcEncodable)]
struct Checksum {
    package: String,
    files: HashMap<String, String>,
}

impl VendorPackage {
    fn new(name: &str) -> VendorPackage {
        VendorPackage {
            p: Some(project(&format!("index/{}", name))),
            cksum: Checksum {
                package: String::new(),
                files: HashMap::new(),
            },
        }
    }

    fn file(&mut self, name: &str, contents: &str) -> &mut VendorPackage {
        self.p = Some(self.p.take().unwrap().file(name, contents));
        self.cksum.files.insert(name.to_string(), cksum(contents.as_bytes()));
        self
    }

    fn build(&mut self) {
        let p = self.p.take().unwrap();
        let json = json::encode(&self.cksum).unwrap();
        let p = p.file(".cargo-checksum.json", &json);
        p.build();
    }
}

#[test]
fn simple() {
    setup();

    VendorPackage::new("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "pub fn foo() {}")
        .build();

    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", r#"
            extern crate foo;

            pub fn bar() {
                foo::foo();
            }
        "#);
    p.build();

    assert_that(p.cargo("build"),
                execs().with_status(0).with_stderr("\
[COMPILING] foo v0.1.0
[COMPILING] bar v0.1.0 ([..]bar)
[FINISHED] [..]
"));
}

#[test]
fn not_there() {
    setup();

    project("index").build();

    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", r#"
            extern crate foo;

            pub fn bar() {
                foo::foo();
            }
        "#);
    p.build();

    assert_that(p.cargo("build"),
                execs().with_status(101).with_stderr("\
error: no matching package named `foo` found (required by `bar`)
location searched: [..]
version required: ^0.1.0
"));
}

#[test]
fn multiple() {
    setup();

    VendorPackage::new("foo-0.1.0")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "pub fn foo() {}")
        .file(".cargo-checksum", "")
        .build();

    VendorPackage::new("foo-0.2.0")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.2.0"
            authors = []
        "#)
        .file("src/lib.rs", "pub fn foo() {}")
        .file(".cargo-checksum", "")
        .build();

    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", r#"
            extern crate foo;

            pub fn bar() {
                foo::foo();
            }
        "#);
    p.build();

    assert_that(p.cargo("build"),
                execs().with_status(0).with_stderr("\
[COMPILING] foo v0.1.0
[COMPILING] bar v0.1.0 ([..]bar)
[FINISHED] [..]
"));
}

#[test]
fn crates_io_then_directory() {
    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", r#"
            extern crate foo;

            pub fn bar() {
                foo::foo();
            }
        "#);
    p.build();

    let cksum = Package::new("foo", "0.1.0")
                        .file("src/lib.rs", "pub fn foo() -> u32 { 0 }")
                        .publish();

    assert_that(p.cargo("build"),
                execs().with_status(0).with_stderr("\
[UPDATING] registry `[..]`
[DOWNLOADING] foo v0.1.0 ([..])
[COMPILING] foo v0.1.0
[COMPILING] bar v0.1.0 ([..]bar)
[FINISHED] [..]
"));

    setup();

    let mut v = VendorPackage::new("foo");
    v.file("Cargo.toml", r#"
        [package]
        name = "foo"
        version = "0.1.0"
        authors = []
    "#);
    v.file("src/lib.rs", "pub fn foo() -> u32 { 1 }");
    v.cksum.package = cksum;
    v.build();

    assert_that(p.cargo("build"),
                execs().with_status(0).with_stderr("\
[COMPILING] foo v0.1.0
[COMPILING] bar v0.1.0 ([..]bar)
[FINISHED] [..]
"));
}

#[test]
fn crates_io_then_bad_checksum() {
    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", "");
    p.build();

    Package::new("foo", "0.1.0").publish();

    assert_that(p.cargo("build"),
                execs().with_status(0));
    setup();

    VendorPackage::new("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "")
        .build();

    assert_that(p.cargo("build"),
                execs().with_status(101).with_stderr("\
error: checksum for `foo v0.1.0` changed between lock files

this could be indicative of a few possible errors:

    * the lock file is corrupt
    * a replacement source in use (e.g. a mirror) returned a different checksum
    * the source itself may be corrupt in one way or another

unable to verify that `foo v0.1.0` is the same as when the lockfile was generated

"));
}

#[test]
fn bad_file_checksum() {
    setup();

    VendorPackage::new("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "")
        .build();

    let mut f = t!(File::create(paths::root().join("index/foo/src/lib.rs")));
    t!(f.write_all(b"fn foo() -> u32 { 0 }"));

    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", "");
    p.build();

    assert_that(p.cargo("build"),
                execs().with_status(101).with_stderr("\
error: the listed checksum of `[..]lib.rs` has changed:
expected: [..]
actual:   [..]

directory sources are not intended to be edited, if modifications are \
required then it is recommended that [replace] is used with a forked copy of \
the source
"));
}
