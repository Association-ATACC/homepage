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
      packages = eachSystem (system:
        let
          pkgs = pkgsFor.${system};
          
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          };
          
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
        in {
          default = rustPlatform.buildRustPackage rec {
            pname = "atacc-homepage";
            version = "0.1.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [
              pkg-config
              cargo-leptos
              binaryen
              wasm-bindgen-cli_0_2_127
              lld
            ];

            buildInputs = with pkgs; [
              sqlite
              openssl
            ];

            buildPhase = ''
              runHook preBuild
              cargo leptos build
              runHook postBuild
            '';

            installPhase = ''
              runHook preInstall
              
              mkdir -p $out/bin $out/share/${pname}
              
              cp target/debug/${pname} $out/bin/
              cp -r target/site $out/share/${pname}/
              
              runHook postInstall
            '';

            doCheck = false; 
          };
        }
      );

      devShells = eachSystem (system: 
        let
          pkgs = pkgsFor.${system};
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          };
        in {
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
      });
    };
}
