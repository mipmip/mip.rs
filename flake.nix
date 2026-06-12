{
  description = "mip";

  inputs = { nixpkgs.url = "github:NixOS/nixpkgs"; };

  outputs = { self, nixpkgs }:
    let
      allSystems = [ "x86_64-linux" "aarch64-linux" ];

      forAllSystems = f:
        nixpkgs.lib.genAttrs allSystems (system:
          f {
            pkgs = import nixpkgs {
              inherit system;
            };
          });
    in {

      packages = forAllSystems ({ pkgs }: {
        default = pkgs.callPackage ./package.nix {};
        mip = pkgs.callPackage ./package.nix {};
      });

      devShells = forAllSystems ({ pkgs }:
        {
          default = with pkgs; mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo gcc cmake pkg-config glib cairo gtk4 webkitgtk_6_0 ];
          buildInputs = with pkgs; [
            rustfmt
            clippy
            pkgs.nodejs
            pkgs.yarn
            gst_all_1.gstreamer
            gst_all_1.gst-plugins-base
            gst_all_1.gst-plugins-good
            gst_all_1.gst-plugins-bad
            gum
          ];

          GST_PLUGIN_PATH = with pkgs.gst_all_1; "${gstreamer}/lib/gstreamer-1.0:${gst-plugins-base}/lib/gstreamer-1.0:${gst-plugins-good}/lib/gstreamer-1.0:${gst-plugins-bad}/lib/gstreamer-1.0";

          # WebKitGTK launches its web processes inside a bwrap sandbox + dbus-proxy.
          # In sandboxed/Nix environments the bundled bwrap helper fails ("Unexpected
          # capabilities but not setuid"), aborting the process. Disable WebKit's
          # internal sandbox so `make run` / `cargo run` launch the viewer.
          WEBKIT_DISABLE_SANDBOX_THIS_IS_DANGEROUS = "1";

          XDG_DATA_DIRS = "${pkgs.gtk4}/share/gsettings-schemas/${pkgs.gtk4.name}:${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${builtins.getEnv "XDG_DATA_DIRS"}";

          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
    };
}
