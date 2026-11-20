{
  version,
  lib,
  rustPlatform,
  wasm-bindgen-cli_0_2_130,
  cargo-leptos,
  binaryen,
  rustc,
  makeWrapper,
  pkg-config,
  sqlite,
  buildFeatures ? [ ],
}:

rustPlatform.buildRustPackage rec {
  pname = "atacc-homepage";
  inherit version;

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

  env.NIX_RELEASE_VERSION = version;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    wasm-bindgen-cli_0_2_130
    cargo-leptos
    binaryen
    rustc.llvmPackages.lld
    makeWrapper
    pkg-config
  ];

  buildInputs = [ sqlite ];

  buildPhase = ''
    runHook preBuild

    cargo leptos build --release

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/bin $out/share/${pname}
    cp target/release/${meta.mainProgram} $out/bin
    cp -r target/site $out/share/${pname}
    wrapProgram $out/bin/${meta.mainProgram} \
      --set LEPTOS_SITE_ROOT $out/share/${pname}/site

    runHook postInstall
  '';

  meta = with lib; {
    description = "Registration website for ATACC, a tutoring and support association, built with Leptos (SSR + hydration) and Axum.";
    mainProgram = "atacc-homepage";
    homepage = "https://github.com/Association-ATACC/homepage";
    license = licenses.mit;
    maintainers = [ maintainers.c2fc2f ];
  };
}
