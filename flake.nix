{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
          ./rust-toolchain.toml;
      in with pkgs; {
        devShells.default = mkShell rec {
          buildInputs = [
            rustToolchain
            wayland
            alsa-lib
            libxkbcommon
            udev
            alsa-lib
            vulkan-loader
            wasm-bindgen-cli
            binaryen
          ];
          nativeBuildInputs = [ pkg-config ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      });
}
