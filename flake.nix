{
  description = "A development environment and build system for the mina-block-explorer project";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
    flake-compat     = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, flake-compat, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            awscli
            cacert
            cargo-audit
            cargo-machete
            cargo-nextest
            leptosfmt
            just
            nodejs_18
            openssl
            pkg-config
            rsync
            rust-analyzer
            toolchain
            trunk
            netcat
          ] ++ lib.optionals stdenv.isDarwin
            [
              darwin.apple_sdk.frameworks.SystemConfiguration
            ]
          ;

          shellHook = ''
            '';
        };
      }
    );
}
