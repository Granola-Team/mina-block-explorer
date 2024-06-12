{
  description =
    "A development environment and build system for the mina-block-explorer project";

  inputs = {
    nixpkgs.url =
      "github:NixOS/nixpkgs?ref=47b604b07d1e8146d5398b42d3306fdebd343986";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, flake-compat, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # used to ensure rustfmt is nightly version to support unstable features
        nightlyToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.minimal.override { extensions = [ "rustfmt" ]; });
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              nightlyToolchain.passthru.availableComponents.rustfmt
              cacert
              cargo-audit
              cargo-machete
              cargo-nextest
              leptosfmt
              just
              nixfmt-classic
              nodejs_18
              nodePackages.pnpm
              openssl
              pkg-config
              rsync
              rust-analyzer
              toolchain
              trunk
              netcat
            ] ++ lib.optionals stdenv.isDarwin
            [ darwin.apple_sdk.frameworks.SystemConfiguration ];

          shellHook = "";
        };
      });
}
