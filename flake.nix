{
  description = "Voice notifier for Claude Code completions";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustVersion = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };
        
        packageName = "voice-notifier";
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustVersion
            rust-analyzer
            clippy
            rustfmt
            cargo-watch
            cargo-edit
            pkg-config
            openssl.dev
          ];
          
          shellHook = ''
            echo "Voice Notifier Development Environment"
            echo "Rust version: $(rustc --version)"
            echo ""
            echo "Don't forget to create a .env file with your ElevenLabs API key!"
          '';

          RUST_BACKTRACE = 1;
          RUST_LOG = "voice_notifier=debug";
        };

        packages.default = rustPlatform.buildRustPackage {
          pname = packageName;
          version = "0.1.0";
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              # Add hash overrides here if needed
            };
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          
          buildInputs = with pkgs; [
            openssl.dev
          ];
        };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };
      }
    );
}