let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-unstable";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in

pkgs.mkShell {
  packages = with pkgs; [
    nodejs_18
    bun
    yarn
  ];

  shellHook = ''
    rm -rf node_modules
    rm yarn.lock
    rm bun.lockb
    bun install
    yarn add sharp #this is a workaround to get sharp install properly
  '';
}