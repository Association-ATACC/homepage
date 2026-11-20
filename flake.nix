{
  description = "ATACC - registration site (Leptos + Axum)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs, ... }:
    let
      systems = nixpkgs.lib.systems.flakeExposed;

      eachSystem =
        f:
        nixpkgs.lib.genAttrs systems (
          system: f system nixpkgs.legacyPackages.${system}
        );
    in
    {
      packages = eachSystem (
        _: pkgs: rec {
          default = atacc-homepage;

          atacc-homepage = pkgs.callPackage ./nix/package.nix {
            version = self.rev or self.dirtyRev or "dirty";
          };
        }
      );

      nixosModules = rec {
        default = atacc-homepage;

        atacc-homepage = import ./nix/nixos-module.nix self;
      };

      devShells = eachSystem (
        system: pkgs: {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.default ];

            packages = with pkgs; [
              rust-analyzer
              rustfmt
              clippy
            ];

            env.RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };
        }
      );

      formatter = eachSystem (_: pkgs: pkgs.callPackage ./treefmt.nix { });
      checks = eachSystem (
        system: pkgs: {
          formatting =
            pkgs.runCommand "check-formatting"
              { nativeBuildInputs = [ self.formatter.${system} ]; }
              ''
                cp -r ${self} src
                chmod -R +w src
                cd src
                treefmt --ci
                touch $out
              '';
        }
      );
    };
}
