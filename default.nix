let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/4521bc6.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rust-nightly = (nixpkgs.rustChannelOf { date = "2021-01-16"; channel = "nightly"; }).rust.override {
    extensions = [
      "rust-src"
      "rust-analysis"
      "rustfmt-preview"
      "clippy-preview"
    ];
    targets = [
      "x86_64-unknown-linux-gnu"
      "wasm32-unknown-unknown"
    ];
  };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "env";
    buildInputs = [
      rust-nightly
      # bashInteractive
      openssl
      pkg-config
    ];
  }
