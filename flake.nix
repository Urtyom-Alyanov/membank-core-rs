{
  description = "Development environment for Membank Core RS";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, utils, git-hooks }:
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

        pre-commit-check = git-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            rustfmt.enable = true;
            rustfmt.package = toolchain;

            clippy.enable = true;
            clippy.package = toolchain;

            nixpkgs-fmt.enable = true;
            trim-trailing-whitespace.enable = true;
          };
        };
      in
      {
        checks = {
          inherit pre-commit-check;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            toolchain
            pkgs.nixpkgs-fmt
          ];

          shellHook = ''
            ${pre-commit-check.shellHook}

            echo "Прошу вас, сделайте мне красиво!"
            cargo --version
          '';
        };
      }
    );
}
