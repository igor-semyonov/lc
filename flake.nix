{
  description = "Flake for NumRS2 devShell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin"];
      perSystem = {
        pkgs,
        system,
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
        devShells.default = let
          fenixPkgs = inputs.fenix.packages.${system};
          # rustToolchain = with fenixPkgs;
          #   combine [
          #     stable.rustc
          #     stable.cargo
          #     stable.clippy
          #     stable.rust-analyzer
          #     stable.rustfmt
          #   ];
          rustToolchain = fenixPkgs.default.toolchain;
        in
          pkgs.mkShell {
            nativeBuildInputs = [
            ];
            buildInputs = [
              rustToolchain
            ];
            # LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          };
      };
    };
}
