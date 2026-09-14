{
  version,
  lib,
  rustPlatform,
  pkg-config,
  cargo-leptos,
  binaryen,
  wasm-bindgen-cli_0_2_127,
  lld,
  buildFeatures ? [ ],
}:

rustPlatform.buildRustPackage rec {
  pname = "atacc-homepage";

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../src
      ../style
      ../public
      ../migrations
      ../build.rs
      ../Cargo.lock
      ../Cargo.toml
    ];
  };

  inherit buildFeatures;
  inherit version;

  # inject version from nix into the build
  env.NIX_RELEASE_VERSION = version;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    pkg-config
    cargo-leptos
    binaryen
    wasm-bindgen-cli_0_2_127
    lld
  ];

  buildInputs = [ ];

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

  meta = with lib; {
    mainProgram = "atacc-homepage";
    homepage = "https://github.com/Association-ATACC/homepage";
    license = licenses.mit;
    maintainers = [ maintainers.c2fc2f ];
  };
}
