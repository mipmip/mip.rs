{ pkgs ? import <nixpkgs> { } }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "mip";
  version = "0.3.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  nativeBuildInputs = with pkgs; [ rustc cargo gcc cmake pkg-config glib cairo webkitgtk_4_1];
  buildInputs = with pkgs; [
    rustfmt
    clippy
    pkgs.nodejs
    pkgs.yarn
    glib
    webkitgtk_4_1
  ];

  #RUST_SRC_PATH = rustPlatform.rustLibSrc;
}
