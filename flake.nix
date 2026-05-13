{
  description = "Development environment for Membank Core RS";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        build-toolchain = fenix.packages.${system}.stable.withComponents [
          "cargo"
          "rustc"
          "rust-src"
          "clippy"
        ];
        toolchain = fenix.packages.${system}.combine [
          build-toolchain
          fenix.packages.${system}.latest.rustfmt
        ];
      in
        {
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [ toolchain ];

            shellHook = ''
              echo "Прошу вас, сделайте мне красиво!"
              cargo --version
            '';
          };
        }
    );
}