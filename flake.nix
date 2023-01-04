{

  description = "embedano";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-22.05";
    utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "utils";
      };
    };
  };

  outputs = inputs@{ self, nixpkgs, utils, rust-overlay, ... }:
    (utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
        # rust = pkgs.rust-bin.stable."1.65.0";
        rust = pkgs.rust-bin.nightly.latest;
        rustDevShell = pkgs.mkShell {
          buildInputs = [
            pkgs.nixpkgs-fmt

            (rust.default.override {
              extensions = [ "rust-analyzer" "rust-src" ];
              # targets: Cortex-M3, Cortex-M4/M7 and Cortex-M4F/M7F
              targets = [
                "thumbv7m-none-eabi"
                "thumbv7em-none-eabi"
                "thumbv7em-none-eabihf"
              ];
            })
          ];
        };

        qemuDevShell = rustDevShell.overrideAttrs (o: {
          buildInputs = o.buildInputs ++ [ pkgs.qemu ];
        });
      in
      {
        devShells = {
          default = rustDevShell;
          withQemu = qemuDevShell;
        };
      })) // {
        herculesCI.ciSystems = [ "x86_64-linux" ];
      };
}
