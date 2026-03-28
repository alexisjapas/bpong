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

        # On définit les "alias" comme de vrais scripts shell exécutables
        customAliases = with pkgs; [
          # -------- Base --------
          (writeShellScriptBin "run" "cargo run")
          (writeShellScriptBin "runr" "cargo run --release")
          (writeShellScriptBin "runo" "cargo run --profile release_optimized")

          # -------- CPU optim --------
          (writeShellScriptBin "run_native" ''RUSTFLAGS="-C target-cpu=native" cargo run'')
          (writeShellScriptBin "runr_native" ''RUSTFLAGS="-C target-cpu=native" cargo run --release'')
          (writeShellScriptBin "runo_native" ''RUSTFLAGS="-C target-cpu=native" cargo run --profile release_optimized'')

          # -------- MangoHud --------
          (writeShellScriptBin "runhud" "mangohud cargo run")
          (writeShellScriptBin "runrhud" "mangohud cargo run --release")
          (writeShellScriptBin "runohud" "mangohud cargo run --profile release_optimized")

          # -------- MangoHud + CPU --------
          (writeShellScriptBin "runhud_native" ''RUSTFLAGS="-C target-cpu=native" mangohud cargo run'')
          (writeShellScriptBin "runrhud_native" ''RUSTFLAGS="-C target-cpu=native" mangohud cargo run --release'')
          (writeShellScriptBin "runohud_native" ''RUSTFLAGS="-C target-cpu=native" mangohud cargo run --profile release_optimized'')
        ];

      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            # On ajoute nos scripts personnalisés aux paquets de l'environnement
            packages = customAliases;

            buildInputs = [
              # Rust dependencies
              (rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; })
              pkg-config
              mangohud
            ]
            ++ lib.optionals (lib.strings.hasInfix "linux" system) [
              alsa-lib
              vulkan-loader
              vulkan-tools
              wayland
              libx11
              libxcursor
              libxi
              libxrandr
              libxkbcommon
              udev
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

            # Le shellHook est désormais nettoyé de ses alias et
            # peut servir uniquement à la vraie configuration du shell.
            shellHook = ''
              echo -e "\033[1;33mRUN COMMANDS\033[0m"
              echo -e "  \033[1mStandard Builds\033[0m"
              printf "    %-18s %s\n" "run"    "Cargo run (debug)"
              printf "    %-18s %s\n" "runr"   "Cargo run (release)"
              printf "    %-18s %s\n" "runo"   "Cargo run (optimized profile)"

              echo -e "\n  \033[1mNative CPU Optimization\033[0m \033[0;90m(RUSTFLAGS=\"-C target-cpu=native\")\033[0m"
              printf "    %-18s %s\n" "run_native"   "Debug build with native CPU instructions"
              printf "    %-18s %s\n" "runr_native"  "Release build with native CPU instructions"
              printf "    %-18s %s\n" "runo_native"  "Optimized build with native CPU instructions"

              echo -e "\n  \033[1mPerformance Monitoring\033[0m \033[0;90m(MangoHud)\033[0m"
              printf "    %-18s %s\n" "runhud"         "Run with MangoHud overlay"
              printf "    %-18s %s\n" "runrhud"        "Run release with MangoHud"
              printf "    %-18s %s\n" "runohud"        "Run optimized with MangoHud"

              echo -e "\n  \033[1mAdvanced / Benchmarking\033[0m"
              printf "    %-18s %s\n" "runhud_native"  "MangoHud + Native CPU optimization"
              printf "    %-18s %s\n" "runrhud_native" "MangoHud + Release + Native CPU"
              printf "    %-18s %s\n" "runohud_native" "MangoHud + Optimized + Native CPU"

              echo -e "\n\033[1;34m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\033[0m"
            '';
          };
      }
    );
}
