# This is a Nix workspace file. It supplies os dependencies.
# If you don't already know how to use it, you probably don't need to worry about it.
with (import <nixpkgs> {});
let
  pkgs = import <nixpkgs> {};
in pkgs.stdenv.mkDerivation rec {
  name = "rules_cargo";
  propagatedBuildInputs = with pkgs; [
    bash
    patchelf
    openssl
    openssl.dev
    pkgconfig
    zlib
    zlib.dev
    cmake
    python
  ];
  shellHook = ''
    # Allow my shell to add custom snippet
    export IS_NIX_SHELL=1
    export BAZEL_SH=/run/current-system/sw/bin/bash
  '';
}
