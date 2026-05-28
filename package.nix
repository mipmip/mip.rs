{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "mip";
  version = "0.3.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = ./.;

  nativeBuildInputs = with pkgs; [ rustc cargo gcc cmake pkg-config glib cairo gtk4 webkitgtk_6_0 ];
  buildInputs = with pkgs; [
    rustfmt
    clippy
    pkgs.nodejs
    pkgs.yarn
    glib
    gtk4
    webkitgtk_6_0
  ];
}
