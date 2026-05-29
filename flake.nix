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
          ];

          GST_PLUGIN_PATH = with pkgs.gst_all_1; "${gstreamer}/lib/gstreamer-1.0:${gst-plugins-base}/lib/gstreamer-1.0:${gst-plugins-good}/lib/gstreamer-1.0";

          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
    };
}
