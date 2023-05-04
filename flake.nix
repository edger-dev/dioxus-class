{
  description = "amateurguitar toolbox";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        toolchain = fenix.packages.${system}.combine [
          fenix.packages.${system}.stable.rustc
          fenix.packages.${system}.stable.cargo
          fenix.packages.${system}.targets.wasm32-unknown-unknown.stable.rust-std
        ];
      in with pkgs; {
        devShell = mkShell {
          buildInputs = [
            # rust toolchain
            toolchain
            # rust native lib
            pkg-config
            openssl
            # utility
            just
            # node tools
            nodejs
            nodePackages.npm
            nodePackages.tailwindcss
          ];
          shellHook = ''
            alias c=cargo
          '';
        };
      }
    );
}
