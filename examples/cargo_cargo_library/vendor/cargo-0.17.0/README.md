Cargo downloads your Rust project’s dependencies and compiles your project.

Learn more at http://doc.crates.io/

## Code Status
[![Build Status](https://travis-ci.org/rust-lang/cargo.svg?branch=master)](https://travis-ci.org/rust-lang/cargo)
[![Build Status](https://ci.appveyor.com/api/projects/status/jnh54531mpidb2c2?svg=true)](https://ci.appveyor.com/project/alexcrichton/cargo)

## Installing Cargo

Cargo is distributed by default with Rust, so if you've got `rustc` installed
locally you probably also have `cargo` installed locally.

## Compiling from Source

Cargo requires the following tools and packages to build:

* `python`
* `curl` (on Unix)
* `cmake`
* OpenSSL headers (only for Unix, this is the `libssl-dev` package on ubuntu)
* `cargo` and `rustc`

First, you'll want to check out this repository

```
git clone --recursive https://github.com/rust-lang/cargo
cd cargo
```

With `cargo` already installed, you can simply run:

```
cargo build --release
```

Otherwise, you can also use a more traditional approach:

```sh
./configure
make
make install
```

More options can be discovered through `./configure`, such as compiling cargo
for more than one target. For example, if you'd like to compile both 32 and 64
bit versions of cargo on unix you would use:

```
$ ./configure --target=i686-unknown-linux-gnu,x86_64-unknown-linux-gnu
```

## Running the tests

To run cargo's tests, use `cargo test`. If you do not have the cross-compilers
installed locally, ignore the cross-compile test failures, or disable them by
using `CFG_DISABLE_CROSS_TESTS=1 cargo test`.

## Adding new subcommands to Cargo

Cargo is designed to be extensible with new subcommands without having to modify
Cargo itself. See [the Wiki page][third-party-subcommands] for more details and
a list of known community-developed subcommands.

[third-party-subcommands]: https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands

## Contributing to the Docs

To contribute to the docs, all you need to do is change the markdown files in
the `src/doc` directory. To view the rendered version of changes you have
made locally, run:

```sh
./configure
make doc
open target/doc/index.html
```

## Releases

High level release notes are available as part of [Rust's release notes][rel].
Cargo releases coincide with Rust releases.

[rel]: https://github.com/rust-lang/rust/blob/master/RELEASES.md

<details>
    <summary>Table of Rust versions with their Cargo versions</summary>

Rust version | Cargo version
-------------|--------------|
   1.12.0    |    0.13.0    |
   1.11.0    |    0.12.0    |
   1.10.0    |    0.11.0    |
   1.9.0     |    0.10.0    |
   1.8.0     |    0.9.0     |
   1.7.0     |    0.8.0     |
   1.6.0     |    0.7.0     |
   1.5.0     |    0.6.0     |
   1.4.0     |    0.5.0     |
   1.3.0     |    0.4.0     |
   1.2.0     |    0.3.0     |
   1.1.0     |    0.2.0     |
   1.0.0     |    0.1.0     |

</details>

## Reporting Issues

Found a bug? We'd love to know about it!

Please report all issues on the github [issue tracker][issues].

[issues]: https://github.com/rust-lang/cargo/issues

## License

Cargo is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

### Third party software

This product includes software developed by the OpenSSL Project
for use in the OpenSSL Toolkit (http://www.openssl.org/).

In binary form, this product includes software that is licensed under the
terms of the GNU General Public License, version 2, with a linking exception,
which can be obtained from the [upstream repository][1].

[1]: https://github.com/libgit2/libgit2

