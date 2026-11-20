# atacc-homepage

The registration website of ATACC, a student tutoring and support association,
written in Rust with [Leptos](https://leptos.dev) (server-side rendering plus
WebAssembly hydration) on top of [Axum](https://github.com/tokio-rs/axum).
Visitors sign up for event notifications through a short form, receive a
confirmation email over SMTP, and validate their address by following the link
it contains. Registrations are stored in a local SQLite database that is
created and migrated automatically on startup.

The user interface and every message shown to visitors are in French.

## Overview

The site serves two pages:

- `/`: the registration form, followed by the weekly schedule of tutoring
  slots.
- `/verify?token=...`: the target of the link sent by email, which confirms
  the address.

When the form is submitted, the server validates the input, stores a pending
registration and emails a confirmation link built from `public_url`. Following
that link marks the registration as verified.

Registering again with an address that is still unverified updates the stored
details, issues a new token (the previous link stops working) and resends the
email. Registering with an address that is already verified changes nothing
and tells the visitor so.

## Installation

### From source

Building requires a stable Rust toolchain with the `wasm32-unknown-unknown`
target, [`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos) and a
`wasm-bindgen-cli` matching the `wasm-bindgen` version pinned in `Cargo.toml`.

```bash
git clone https://github.com/Association-ATACC/homepage
cd homepage
cargo leptos build --release
```

This produces the server binary at `target/release/atacc-homepage` and the
static site (WebAssembly bundle, JavaScript glue, CSS, and the contents of
`public/`) under `target/site/`.

### With Nix

A Nix flake is provided:

```bash
nix run github:Association-ATACC/homepage
# or
nix build
# or, to enter a development shell:
nix develop
```

The package wraps the binary so that `LEPTOS_SITE_ROOT` points to the built
site inside the Nix store. The flake also exposes a NixOS module, see
[Deployment on NixOS](#deployment-on-nixos).

## Usage

### Development

```bash
nix develop
cp config.example.toml config.toml
$EDITOR config.toml   # at least the SMTP settings
cargo leptos watch
```

The site is then available at <http://127.0.0.1:3000>, with live reload.

### Production

On NixOS, use the [module](#deployment-on-nixos). Otherwise, build the site as
described in [From source](#from-source), then start the binary with the same
environment the module sets:

```bash
ATACC_CONFIG=/etc/atacc/config.toml \
LEPTOS_SITE_ROOT=target/site \
LEPTOS_SITE_ADDR=127.0.0.1:3000 \
LEPTOS_ENV=PROD \
target/release/atacc-homepage
```

The server speaks plain HTTP and is meant to sit behind a reverse proxy that
terminates TLS. If the configuration file cannot be read or parsed, it prints
the error and exits with status 1.

## Configuration

Copy [`config.example.toml`](config.example.toml) to `config.toml` and adjust
it. The file contains an SMTP password in clear text and must **never** be
committed.

| Section      | Field          | Description                                                                                   | Default      |
| ------------ | -------------- | --------------------------------------------------------------------------------------------- | ------------ |
| `[server]`   | `public_url`   | Public base URL of the site, used to build the link sent by email (a trailing `/` is ignored) | *(required)* |
| `[database]` | `path`         | Path of the SQLite file, created and migrated on startup                                      | *(required)* |
| `[smtp]`     | `host`         | SMTP server hostname                                                                          | *(required)* |
| `[smtp]`     | `port`         | SMTP server port                                                                              | *(required)* |
| `[smtp]`     | `username`     | SMTP username                                                                                 | *(required)* |
| `[smtp]`     | `password`     | SMTP password; when omitted, the server is contacted without authentication                   | *(none)*     |
| `[smtp]`     | `from_address` | Sender of the emails, with an optional display name (`ATACC <no-reply@atacc.org>`)            | *(required)* |
| `[smtp]`     | `security`     | Transport security, see below                                                                 | `starttls`   |

`public_url` is the only place to change when the site moves to another
domain.

### SMTP security

| Value      | Behavior                                                 | Typical port |
| ---------- | -------------------------------------------------------- | ------------ |
| `starttls` | Plain connection upgraded to TLS with `STARTTLS`         | `587`        |
| `smtps`    | Implicit TLS, the connection is encrypted from the start | `465`        |
| `none`     | No encryption, only suitable for local or trusted relays | `25`, `1025` |

TLS is handled by `rustls`: no system OpenSSL is required.

### Environment variables

| Variable           | Description                         | Default          |
| ------------------ | ----------------------------------- | ---------------- |
| `ATACC_CONFIG`     | Path of the TOML configuration file | `config.toml`    |
| `LEPTOS_SITE_ADDR` | Address the server binds to         | `127.0.0.1:3000` |
| `LEPTOS_SITE_ROOT` | Directory holding the built site    | `target/site`    |
| `LEPTOS_ENV`       | `DEV` or `PROD`                     | `DEV`            |

## Deployment on NixOS

The flake exports a NixOS module, `nixosModules.default`, that runs the site
as a hardened systemd service.

```nix
{
  inputs.atacc-homepage.url = "github:Association-ATACC/homepage";

  outputs =
    { nixpkgs, atacc-homepage, ... }:
    {
      nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          atacc-homepage.nixosModules.default
          {
            services.atacc-homepage = {
              enable = true;
              publicUrl = "https://inscription.atacc.org";
              smtp = {
                host = "smtp.example.org";
                username = "no-reply@atacc.org";
                passwordFile = "/run/secrets/atacc-smtp-password";
                fromAddress = "ATACC <no-reply@atacc.org>";
              };
            };

            # TLS termination in front of the service
            services.nginx.virtualHosts."inscription.atacc.org" = {
              enableACME = true;
              forceSSL = true;
              locations."/".proxyPass = "http://127.0.0.1:3000";
            };
          }
        ];
      };
    };
}
```

### Options

| Option              | Type         | Description                                                     | Default           |
| ------------------- | ------------ | --------------------------------------------------------------- | ----------------- |
| `enable`            | bool         | Enable the service                                              | `false`           |
| `package`           | package      | Server binary and built site                                    | flake's `default` |
| `host`              | string       | Address the server binds to                                     | `127.0.0.1`       |
| `port`              | port         | Port the server listens on                                      | `3000`            |
| `openFirewall`      | bool         | Open `port` in the firewall                                     | `false`           |
| `publicUrl`         | string       | Public URL of the site, without trailing slash                  | *(required)*      |
| `smtp.host`         | string       | SMTP server hostname                                            | *(required)*      |
| `smtp.port`         | port         | SMTP server port                                                | `587`             |
| `smtp.username`     | string       | SMTP username                                                   | *(required)*      |
| `smtp.passwordFile` | null or path | File containing the SMTP password, `null` for no authentication | *(required)*      |
| `smtp.fromAddress`  | string       | `From:` address of the emails                                   | *(required)*      |
| `smtp.security`     | enum         | `none`, `smtps` or `starttls`                                   | `starttls`        |

- Give `passwordFile` as a **string** (for example a path managed by agenix or
  sops-nix), not as a Nix path literal, which would copy the secret into the
  world-readable Nix store. The password is only read at service start.
- The database lives at `/var/lib/atacc-homepage/atacc.db`: that is the file
  to back up.

## Implementation Notes

### Input validation

Validation runs on the server, in `src/pages/home/logic.rs`. The form also
refuses to submit while a field is empty, so every field is required.

| Field          | Rule                                                                  |
| -------------- | --------------------------------------------------------------------- |
| Names          | Non-empty after trimming                                              |
| Email          | Must look like `local@domain.tld`                                     |
| Student number | Exactly 8 digits                                                      |
| Phone          | French local (`0` + 9 digits) or international (`+` + 9 to 15 digits) |

Spaces, dots and dashes are removed from the phone number, and a French local
number such as `06 12 34 56 78` is stored as `+33612345678`.

### Data model

A single table, defined in
[`migrations/0001_init.sql`](migrations/0001_init.sql):

| Column               | Description                                           |
| -------------------- | ----------------------------------------------------- |
| `id`                 | UUID v4, primary key                                  |
| `first_names`        | First name(s)                                         |
| `last_names`         | Last name(s)                                          |
| `email`              | Email address, unique                                 |
| `phone`              | Phone number, normalized to international format      |
| `student_number`     | Student number                                        |
| `verification_token` | UUID v4 sent in the confirmation link (indexed)       |
| `verified`           | `0` until the link is followed, then `1`              |
| `created_at`         | RFC 3339 timestamp of the first registration          |
| `verified_at`        | RFC 3339 timestamp of the confirmation, `NULL` before |

Queries use SQLx's runtime API (`query`, `query_as`), so no database or
offline query cache is needed to compile the project. Migrations are embedded
in the binary and applied at startup.

### Weekly schedule

The schedule is currently static: `current_slots()` in
[`src/pages/home/components/weekly_schedule.rs`](src/pages/home/components/weekly_schedule.rs)
returns a hard-coded vector of `Slot` values (day, start, end, subject). Edit
it and rebuild to change the schedule. When the list is empty, the page shows
an explanatory message instead of the grid.

## License

This project is licensed under the [MIT License](LICENSE).
