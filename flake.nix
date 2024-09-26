{
  description = "A development environment and build system for the mina-block-explorer project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=931494da4b60fb26719e231d6de4b2c96167a1ce";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    flake-compat,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};
      toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      # used to ensure rustfmt is nightly version to support unstable features
      nightlyToolchain =
        pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.minimal.override {extensions = ["rustfmt"];});
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs;
          [
            nightlyToolchain.passthru.availableComponents.rustfmt
            alejandra
            cacert
            cargo-audit
            cargo-machete
            cargo-nextest
            leptosfmt
            just
            nodejs_22
            nodePackages.pnpm
            openssl
            pkg-config
            ruby
            rubyPackages.standard
            rust-analyzer
            toolchain
            trunk
          ]
          ++ lib.optionals stdenv.isDarwin
          [darwin.apple_sdk.frameworks.SystemConfiguration];

        shellHook = "";
      };
    });
}
