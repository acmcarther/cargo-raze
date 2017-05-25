#[macro_use]
extern crate cargotest;
extern crate flate2;
extern crate hamcrest;
extern crate tar;
extern crate url;

use std::io::prelude::*;
use std::fs::{self, File};
use std::io::SeekFrom;
use std::path::PathBuf;

use cargotest::support::git::repo;
use cargotest::support::paths;
use cargotest::support::{project, execs};
use flate2::read::GzDecoder;
use hamcrest::assert_that;
use tar::Archive;
use url::Url;

fn registry_path() -> PathBuf { paths::root().join("registry") }
fn registry() -> Url { Url::from_file_path(&*registry_path()).ok().unwrap() }
fn upload_path() -> PathBuf { paths::root().join("upload") }
fn upload() -> Url { Url::from_file_path(&*upload_path()).ok().unwrap() }

fn setup() {
    let config = paths::root().join(".cargo/config");
    t!(fs::create_dir_all(config.parent().unwrap()));
    t!(t!(File::create(&config)).write_all(&format!(r#"
        [registry]
            token = "api-token"
    "#).as_bytes()));
    t!(fs::create_dir_all(&upload_path().join("api/v1/crates")));

    repo(&registry_path())
        .file("config.json", &format!(r#"{{
            "dl": "{0}",
            "api": "{0}"
        }}"#, upload()))
        .build();
}

#[test]
fn simple() {
    setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
        "#)
        .file("src/main.rs", "fn main() {}");

    assert_that(p.cargo_process("publish").arg("--no-verify")
                 .arg("--host").arg(registry().to_string()),
                execs().with_status(0).with_stderr(&format!("\
[UPDATING] registry `{reg}`
[WARNING] manifest has no documentation, [..]
See [..]
[PACKAGING] foo v0.0.1 ({dir})
[UPLOADING] foo v0.0.1 ({dir})
",
        dir = p.url(),
        reg = registry())));

    let mut f = File::open(&upload_path().join("api/v1/crates/new")).unwrap();
    // Skip the metadata payload and the size of the tarball
    let mut sz = [0; 4];
    assert_eq!(f.read(&mut sz).unwrap(), 4);
    let sz = ((sz[0] as u32) <<  0) |
             ((sz[1] as u32) <<  8) |
             ((sz[2] as u32) << 16) |
             ((sz[3] as u32) << 24);
    f.seek(SeekFrom::Current(sz as i64 + 4)).unwrap();

    // Verify the tarball
    let mut rdr = GzDecoder::new(f).unwrap();
    assert_eq!(rdr.header().filename().unwrap(), "foo-0.0.1.crate".as_bytes());
    let mut contents = Vec::new();
    rdr.read_to_end(&mut contents).unwrap();
    let mut ar = Archive::new(&contents[..]);
    for file in ar.entries().unwrap() {
        let file = file.unwrap();
        let fname = file.header().path_bytes();
        let fname = &*fname;
        assert!(fname == b"foo-0.0.1/Cargo.toml" ||
                fname == b"foo-0.0.1/src/main.rs",
                "unexpected filename: {:?}", file.header().path());
    }
}

#[test]
fn git_deps() {
    setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"

            [dependencies.foo]
            git = "git://path/to/nowhere"
        "#)
        .file("src/main.rs", "fn main() {}");

    assert_that(p.cargo_process("publish").arg("-v").arg("--no-verify")
                 .arg("--host").arg(registry().to_string()),
                execs().with_status(101).with_stderr("\
[UPDATING] registry [..]
[ERROR] crates cannot be published to crates.io with dependencies sourced from \
a repository\neither publish `foo` as its own crate on crates.io and \
specify a crates.io version as a dependency or pull it into this \
repository and specify it with a path and version\n\
(crate `foo` has repository path `git://path/to/nowhere`)\
"));
}

#[test]
fn path_dependency_no_version() {
    setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"

            [dependencies.bar]
            path = "bar"
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.0.1"
            authors = []
        "#)
        .file("bar/src/lib.rs", "");

    assert_that(p.cargo_process("publish")
                 .arg("--host").arg(registry().to_string()),
                execs().with_status(101).with_stderr("\
[UPDATING] registry [..]
[ERROR] all path dependencies must have a version specified when publishing.
dependency `bar` does not specify a version
"));
}

#[test]
fn unpublishable_crate() {
    setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            publish = false
        "#)
        .file("src/main.rs", "fn main() {}");

    assert_that(p.cargo_process("publish")
                 .arg("--host").arg(registry().to_string()),
                execs().with_status(101).with_stderr("\
[ERROR] some crates cannot be published.
`foo` is marked as unpublishable
"));
}

#[test]
fn dont_publish_dirty() {
    setup();

    repo(&paths::root().join("foo"))
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .file("src/main.rs", "fn main() {}")
        .build();

    let p = project("foo");
    t!(File::create(p.root().join("bar")));
    assert_that(p.cargo("publish")
                 .arg("--host").arg(registry().to_string()),
                execs().with_status(101).with_stderr("\
[UPDATING] registry `[..]`
error: 1 dirty files found in the working directory:

bar

to proceed despite this, pass the `--allow-dirty` flag
"));
}

#[test]
fn publish_clean() {
    setup();

    repo(&paths::root().join("foo"))
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .file("src/main.rs", "fn main() {}")
        .build();

    let p = project("foo");
    assert_that(p.cargo("publish")
                 .arg("--host").arg(registry().to_string()),
                execs().with_status(0));
}

#[test]
fn publish_in_sub_repo() {
    setup();

    repo(&paths::root().join("foo"))
        .file("bar/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .file("bar/src/main.rs", "fn main() {}")
        .build();

    let p = project("foo");
    t!(File::create(p.root().join("baz")));
    assert_that(p.cargo("publish").cwd(p.root().join("bar"))
                 .arg("--host").arg(registry().to_string()),
                execs().with_status(0));
}

#[test]
fn publish_when_ignored() {
    setup();

    repo(&paths::root().join("foo"))
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .file("src/main.rs", "fn main() {}")
        .file(".gitignore", "baz")
        .build();

    let p = project("foo");
    t!(File::create(p.root().join("baz")));
    assert_that(p.cargo("publish")
                 .arg("--host").arg(registry().to_string()),
                execs().with_status(0));
}

#[test]
fn ignore_when_crate_ignored() {
    setup();

    repo(&paths::root().join("foo"))
        .file(".gitignore", "bar")
        .nocommit_file("bar/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .nocommit_file("bar/src/main.rs", "fn main() {}");
    let p = project("foo");
    t!(File::create(p.root().join("bar/baz")));
    assert_that(p.cargo("publish").cwd(p.root().join("bar"))
                 .arg("--host").arg(registry().to_string()),
                execs().with_status(0));
}

#[test]
fn new_crate_rejected() {
    setup();

    repo(&paths::root().join("foo"))
        .nocommit_file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .nocommit_file("src/main.rs", "fn main() {}");
    let p = project("foo");
    t!(File::create(p.root().join("baz")));
    assert_that(p.cargo("publish")
                 .arg("--host").arg(registry().to_string()),
                execs().with_status(101));
}

#[test]
fn dry_run() {
    setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
        "#)
        .file("src/main.rs", "fn main() {}");

    assert_that(p.cargo_process("publish").arg("--dry-run")
                 .arg("--host").arg(registry().to_string()),
                execs().with_status(0).with_stderr(&format!("\
[UPDATING] registry `[..]`
[WARNING] manifest has no documentation, [..]
See [..]
[PACKAGING] foo v0.0.1 ({dir})
[VERIFYING] foo v0.0.1 ({dir})
[COMPILING] foo v0.0.1 [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[UPLOADING] foo v0.0.1 ({dir})
[WARNING] aborting upload due to dry run
",
        dir = p.url())));

    // Ensure the API request wasn't actually made
    assert!(!upload_path().join("api/v1/crates/new").exists());
}
