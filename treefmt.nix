{
  lib,
  ghc,
  formats,
  treefmt,
  rustfmt,
  taplo,
  mdformat,
  biome,
  deadnix,
  nixfmt,
}:
let
  haskellAvailable = ghc.bootstrapAvailable;

  taploConfig = (formats.toml { }).generate "taplo.toml" {
    formatting = {
      compact_arrays = false;
      array_auto_collapse = false;
      array_auto_expand = true;
      array_trailing_comma = true;
    };
  };
in
treefmt.withConfig {
  runtimeInputs = [
    rustfmt
    taplo
    (mdformat.withPlugins (p: [ p.mdformat-gfm ]))
    biome
    deadnix
  ]
  ++ lib.optionals haskellAvailable [ nixfmt ];

  settings = {
    verbose = 1;
    on-unmatched = "info";
    tree-root-file = "flake.nix";

    formatter = {
      rustfmt = {
        command = "rustfmt";
        includes = [ "*.rs" ];
      };

      taplo = {
        command = "taplo";
        options = [
          "format"
          "--config"
          "${taploConfig}"
        ];
        includes = [ "*.toml" ];
      };

      mdformat = {
        command = "mdformat";
        options = [
          "--wrap"
          "78"
        ];
        includes = [ "*.md" ];
        priority = 2;
      };

      biome-css = {
        command = "biome";
        options = [
          "format"
          "--write"
          "--no-errors-on-unmatched"
          "--css-formatter-enabled=true"
          "--indent-style=space"
          "--indent-width=2"
          "--line-width=80"
        ];
        includes = [ "*.css" ];
      };

      deadnix = {
        command = "deadnix";
        options = [ "--edit" ];
        includes = [ "*.nix" ];
        priority = 1;
      };
    }
    // lib.optionalAttrs haskellAvailable {
      nixfmt = {
        command = "nixfmt";
        options = [
          "--width=70"
          "--strict"
        ];
        includes = [ "*.nix" ];
        priority = 2;
      };
    };
  };
}
