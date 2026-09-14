self:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.services.atacc-homepage;

  stateDirectoryName = "atacc-homepage";
  runtimeDirectoryName = "atacc-homepage";

  runtimeConfigPath = "/run/${runtimeDirectoryName}/config.toml";
  databasePath = "/var/lib/${stateDirectoryName}/atacc.db";

  smtpPasswordPlaceholder = "@smtp-password@";

  configTemplate = pkgs.writeText "atacc-homepage-config.toml.template" ''
    [server]
    public_url = ${builtins.toJSON cfg.publicUrl}

    [database]
    path = ${builtins.toJSON databasePath}

    [smtp]
    host = ${builtins.toJSON cfg.smtp.host}
    port = ${toString cfg.smtp.port}
    username = ${builtins.toJSON cfg.smtp.username}
    password = "${smtpPasswordPlaceholder}"
    from_address = ${builtins.toJSON cfg.smtp.fromAddress}
    use_starttls = ${lib.boolToString cfg.smtp.useStarttls}
  '';
in
{
  options.services.atacc-homepage = {
    enable = lib.mkEnableOption "the ATACC registration website";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      defaultText = lib.literalExpression "atacc-homepage.packages.<system>.default";
      description = "The atacc-homepage package (server binary + built site) to run.";
    };

    host = lib.mkOption {
      type = lib.types.str;
      default = "127.0.0.1";
      description = ''
        Address the Axum server binds to. Left on localhost by default;
        put a reverse proxy in front rather than exposing this directly.
      '';
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 3000;
      description = "Port the Axum server listens on.";
    };

    openFirewall = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = "Whether to open `port` in the firewall.";
    };

    publicUrl = lib.mkOption {
      type = lib.types.str;
      example = "https://inscription.atacc.org";
      description = ''
        Public URL of the site (no trailing slash), used to build the
        confirmation link sent by email. This can differ from
        `host`/`port` when the service sits behind a reverse proxy.
      '';
    };

    smtp = {
      host = lib.mkOption {
        type = lib.types.str;
        example = "smtp.example.org";
        description = "SMTP server hostname used to send verification emails.";
      };

      port = lib.mkOption {
        type = lib.types.port;
        default = 587;
        description = "SMTP server port.";
      };

      username = lib.mkOption {
        type = lib.types.str;
        example = "no-reply@atacc.org";
        description = "SMTP username.";
      };

      passwordFile = lib.mkOption {
        type = lib.types.path;
        example = "/run/secrets/atacc-smtp-password";
        description = ''
          Path to a file containing the SMTP password (plain text, no
          quoting needed).

          Pass this as a plain string, NOT a Nix path literal (`./secret`
          or `/nix/store/...`) — a path literal gets copied into the
          world-readable Nix store. As a string, it's only ever read at
          service start, via systemd's `LoadCredential=`. Works well with
          agenix, sops-nix, or a plain root-only file.
        '';
      };

      fromAddress = lib.mkOption {
        type = lib.types.str;
        example = "ATACC <no-reply@atacc.org>";
        description = "From: address (and optional display name) used for outgoing emails.";
      };

      useStarttls = lib.mkOption {
        type = lib.types.bool;
        default = true;
        description = ''
          `true` for STARTTLS (generally port 587), `false` for a
          connection that's encrypted from the start (generally port 465).
        '';
      };
    };
  };

  config = lib.mkIf cfg.enable {
    networking.firewall.allowedTCPPorts = lib.mkIf cfg.openFirewall [ cfg.port ];

    systemd.services.atacc-homepage = {
      description = "ATACC registration website";
      after = [ "network-online.target" ];
      wants = [ "network-online.target" ];
      wantedBy = [ "multi-user.target" ];

      environment = {
        ATACC_CONFIG = runtimeConfigPath;

        LEPTOS_OUTPUT_NAME = "atacc-homepage";
        LEPTOS_SITE_ROOT = "${cfg.package}/share/atacc-homepage/site";
        LEPTOS_SITE_PKG_DIR = "pkg";
        LEPTOS_SITE_ADDR = "${cfg.host}:${toString cfg.port}";
        LEPTOS_ENV = "PROD";
      };

      preStart = ''
        ${pkgs.coreutils}/bin/install -m 0600 ${configTemplate} ${runtimeConfigPath}
        ${lib.getExe pkgs.replace-secret} \
          '${smtpPasswordPlaceholder}' \
          "$CREDENTIALS_DIRECTORY/smtp-password" \
          ${runtimeConfigPath}
      '';

      serviceConfig = {
        ExecStart = lib.getExe cfg.package;
        Restart = "on-failure";
        RestartSec = "5s";

        DynamicUser = true;
        StateDirectory = stateDirectoryName;
        RuntimeDirectory = runtimeDirectoryName;
        RuntimeDirectoryMode = "0700";

        LoadCredential = [ "smtp-password:${cfg.smtp.passwordFile}" ];

        NoNewPrivileges = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        PrivateTmp = true;
        PrivateDevices = true;
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectKernelLogs = true;
        ProtectControlGroups = true;
        ProtectClock = true;
        ProtectHostname = true;
        RestrictAddressFamilies = [
          "AF_UNIX"
          "AF_INET"
          "AF_INET6"
        ];
        RestrictNamespaces = true;
        RestrictSUIDSGID = true;
        RestrictRealtime = true;
        LockPersonality = true;
        RemoveIPC = true;
        UMask = "0077";
        SystemCallFilter = [ "@system-service" ];
      }
      // lib.optionalAttrs (cfg.port < 1024) {
        AmbientCapabilities = [ "CAP_NET_BIND_SERVICE" ];
        CapabilityBoundingSet = [ "CAP_NET_BIND_SERVICE" ];
      };
    };
  };
}
