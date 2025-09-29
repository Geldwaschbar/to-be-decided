{
  description = "macroquad flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs @ {
    self,
    flake-parts,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [inputs.pre-commit.flakeModule];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem = {
        config,
        self',
        inputs',
        system,
        ...
      }: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };
        bi = with pkgs; [
          openssl
          pkg-config
          (rust-bin.stable.latest.default.override {
            extensions = ["rust-src"];
            targets = [
              "aarch64-apple-darwin"
              "x86_64-unknown-linux-gnu"
              "wasm32-unknown-unknown"
            ];
          })
          rust-analyzer
        ];
        devDependencies = with pkgs; [
          bacon
          sccache
          wasm-pack
          nodejs
          tiled
        ];
        libDependencies = with pkgs; [
          libGL
          libxkbcommon
          xorg.libX11
          xorg.libXi
        ];
      in
        with pkgs; {
          pre-commit.settings.hooks = {
            alejandra.enable = true;
            rustfmt.enable = true;
          };
          devShells.default = pkgs.mkShell {
            buildInputs = bi ++ devDependencies ++ libDependencies;
            shellHook = ''
              export LD_LIBRARY_PATH=${lib.makeLibraryPath libDependencies}:$LD_LIBRARY_PATH

              ${config.pre-commit.installationScript}
            '';
          };
          packages = {
            default = rustPlatform.buildRustPackage rec {
              pname = "macroquad-template";
              version = "0.1.0";
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
              # cargoBuildFlags = ["--package ${pname}"];
              checkType = "debug";
              nativeBuildInputs = [pkgs.pkg-config];
              buildInputs = bi;
            };
          };
        };
    };
}
