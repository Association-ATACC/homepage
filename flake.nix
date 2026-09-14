{
  description = "ATACC - registration site (Leptos + Axum)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
      rust-overlay,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      eachSystem = lib.genAttrs (import systems);

      pkgsFor = eachSystem (
        system:
        import nixpkgs {
          localSystem = system;
          overlays = [ (import rust-overlay) ];
        }
      );
    in
    {
      packages = eachSystem (
        system:
        let
          pkgs = pkgsFor.${system};

          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          };

          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
        in
        {
          default = self.packages.${system}.kag;

          kag = pkgsFor.${system}.callPackage ./nix/package.nix {
            inherit rustPlatform;

            version = self.rev or self.dirtyRev or "dirty";
          };
        }
      );

      nixosModules = {
        default = self.nixosModules.atacc-homepage;
        atacc-homepage = import ./nix/nixos-module.nix self;
      };

      devShells = eachSystem (
        system:
        let
          pkgs = pkgsFor.${system};
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          };
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.default ];

            env = {
              RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            };

            nativeBuildInputs = with pkgs; [
              rustToolchain
              rust-analyzer
            ];
          };
        }
      );
    };
}
