{ pkgs ? import <nixpkgs> { } }:

let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = "mip";
  version = cargoToml.package.version;
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
    gst_all_1.gst-plugins-bad
  ];

  postInstall = ''
    mkdir -p $out/share/icons/hicolor/scalable/apps
    cp icons/mip-icon.svg $out/share/icons/hicolor/scalable/apps/mip.svg
    mkdir -p $out/share/applications
    cp mip.desktop $out/share/applications/mip.desktop
  '';

  preFixup = ''
    gappsWrapperArgs+=(
      --prefix GST_PLUGIN_PATH : "${pkgs.gst_all_1.gstreamer}/lib/gstreamer-1.0:${pkgs.gst_all_1.gst-plugins-base}/lib/gstreamer-1.0:${pkgs.gst_all_1.gst-plugins-good}/lib/gstreamer-1.0:${pkgs.gst_all_1.gst-plugins-bad}/lib/gstreamer-1.0"
      # WebKitGTK launches its web processes inside a bwrap sandbox + dbus-proxy.
      # In sandboxed/Nix environments the bundled bwrap helper fails ("Unexpected
      # capabilities but not setuid"), which aborts the process. Disable WebKit's
      # internal sandbox so the viewer launches. --set-default lets a user override.
      --set-default WEBKIT_DISABLE_SANDBOX_THIS_IS_DANGEROUS 1
    )
  '';
}
