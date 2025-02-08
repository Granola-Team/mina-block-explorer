{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    fenix,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: {
      packages.default = let
        toolchain = with fenix.packages.${system};
          combine [
            default.toolchain
            targets.wasm32-unknown-unknown.latest.rust-std
          ];
      in
        toolchain;
    });
}
