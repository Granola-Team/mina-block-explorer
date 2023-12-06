{
  description = "A development environment and build system for mina-block-explorer";

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
      in
      with pkgs;
      {
        devShells.default = mkShell {
          nativeBuildInputs = [
            playwright-driver.browsers
          ];
          buildInputs = [
            cacert
            cargo-audit
            cargo-machete
            cargo-nextest
            just
            nodejs_18
            openssl
            pkg-config
            playwright-test
            # playwright-driver.browsers
            trunk
            (rust-bin.selectLatestNightlyWith( toolchain: toolchain.default.override {
              extensions= [ "rust-src" "rust-analyzer" ];
              targets = [ "wasm32-unknown-unknown" ];
            }))
          ] ++ lib.optionals stdenv.isDarwin
            [
              darwin.apple_sdk.frameworks.SystemConfiguration
            ]
          ;

          shellHook = ''
            export PLAYWRIGHT_BROWSERS_PATH=${pkgs.playwright-driver.browsers.outPath}
            export PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS=true
          '';
        };
      }
    );
}
