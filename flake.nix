{
  description = "notmuch-rs";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {

      systems = inputs.nixpkgs.lib.systems.flakeExposed;

      perSystem =
        { system, ... }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
            config.allowUnfree = true;
          };
          rustToolchain = pkgs.rust-bin.beta.latest.default.override {
            extensions = [
              "rust-src"
              "rust-std"
              "rust-analyzer"
              "clippy"
              "rustfmt"
            ];
          };
        in
        {
          devShells.default = pkgs.mkShell {
            packages = [
              rustToolchain
              pkgs.notmuch
              pkgs.pkg-config
            ];
          };
        };
    };
}
