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
        rust = pkgs.rust-bin.nightly.latest.default;
        rustDevShell = pkgs.mkShell {
          buildInputs = [
            pkgs.nixpkgs-fmt

            (rust.override {
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

        checks = {
          qemu-example-check = pkgs.runCommand "test-command"
            {
              buildInputs = [ rust ];
            }
            ''
              echo $(cargo version)
              cd ${./.}/qemu-example
              cargo fmt --check
              touch $out
            '';

          sdk-check = pkgs.runCommand "test-command"
            {
              buildInputs = [ rust ];
            }
            ''
              echo $(cargo version)
              cd ${./.}/cardano-embedded-sdk
              cargo fmt --check
              touch $out
            '';
            
        };

      })) // {
      herculesCI.ciSystems = [ "x86_64-linux" ];
    };
}
