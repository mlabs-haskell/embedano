{

  description = "embedano";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-22.05";
    utils.url = "github:numtide/flake-utils";
    cardano-node.url = "github:input-output-hk/cardano-node/1.35.4";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "utils";
      };
    };
  };

  outputs = inputs@{ self, nixpkgs, utils, rust-overlay, cardano-node, ... }:
    (utils.lib.eachSystem [ utils.lib.system.x86_64-linux utils.lib.system.x86_64-darwin] (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
        # rust = pkgs.rust-bin.stable."1.65.0";
        rust = pkgs.rust-bin.nightly.latest.default;
        rustDevShell = pkgs.mkShell {
          buildInputs = [
            cardano-node.packages.${system}.cardano-cli
            cardano-node.packages.${system}.bech32
            pkgs.nixpkgs-fmt
            pkgs.pkg-config
            pkgs.libusb

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
              cd ${./.}/examples/qemu-example
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
