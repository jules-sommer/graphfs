# in flake.nix
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
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        # new! 👇
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];
        # also new! 👇
        buildInputs = with pkgs; [
          openssl
          pkg-config
          openssl.dev
          wayland
          wayland.dev
          zlib
          glib
          libxkbcommon
          libxkbcommon.dev
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          # 👇 and now we can just inherit them
          inherit buildInputs nativeBuildInputs;
          LD_LIBRARY_PATH = lib.makeLibraryPath (
            [
              # add some stuffs here
            ]
            ++ buildInputs
            ++ nativeBuildInputs
          );
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          OPENSSL_DIR = "${pkgs.openssl.out}";
          OPENSSL_LIB_DIR = lib.makeLibraryPath [ pkgs.openssl ];
        };
      }
    );
}
