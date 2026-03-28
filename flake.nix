{
  description = "bevy flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs =
              [
                # Rust dependencies
                (rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; })
                pkg-config
                mangohud
              ]
              ++ lib.optionals (lib.strings.hasInfix "linux" system) [
                # for Linux
                # Audio (Linux only)
                alsa-lib
                # Cross Platform 3D Graphics API
                vulkan-loader
                # For debugging around vulkan
                vulkan-tools
                # Wayland
                wayland
                # X11
                libx11
                libxcursor
                libxi
                libxrandr
                libxkbcommon
                # Misc
                libudev-zero
              ];
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            LD_LIBRARY_PATH = lib.makeLibraryPath [
              vulkan-loader
              wayland
              libx11
              libxi
              libxcursor
              libxkbcommon
            ];
          };
      }
    );
}