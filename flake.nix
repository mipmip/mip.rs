{
  description = "mip";

  inputs = { nixpkgs.url = "github:NixOS/nixpkgs"; };

  outputs = { self, nixpkgs }:
    let
      allSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];

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
            #          nativeBuildInputs = with pkgs; [ rustc cargo gcc cmake pkg-config glib cairo webkitgtk webkitgtk_4_1];
          nativeBuildInputs = with pkgs; [ rustc cargo gcc cmake pkg-config glib cairo webkitgtk_4_1];
          buildInputs = with pkgs; [
            rustfmt
            clippy
            pkgs.nodejs
            pkgs.yarn
          ];

          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          shellHook = ''
            export WEBKIT_DISABLE_DMABUF_RENDERER=1
            /usr/bin/env zsh
          '';
        };
      });
    };
}
