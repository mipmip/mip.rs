{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "mip";
  version = "0.3.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = ./.;

  nativeBuildInputs = with pkgs; [ rustc cargo gcc cmake pkg-config glib cairo gtk4 webkitgtk_6_0 pkgs.wrapGAppsHook4 ];
  buildInputs = with pkgs; [
    rustfmt
    clippy
    pkgs.nodejs
    pkgs.yarn
    glib
    gtk4
    webkitgtk_6_0
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
    gst_all_1.gst-plugins-good
  ];

  preFixup = ''
    gappsWrapperArgs+=(
      --prefix GST_PLUGIN_PATH : "${pkgs.gst_all_1.gstreamer}/lib/gstreamer-1.0:${pkgs.gst_all_1.gst-plugins-base}/lib/gstreamer-1.0:${pkgs.gst_all_1.gst-plugins-good}/lib/gstreamer-1.0"
    )
  '';
}
