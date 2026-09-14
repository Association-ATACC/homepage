# ATACC — site d'inscription

Site d'inscription à l'ATACC (Association de Tutorat et d'Aide), en Rust
avec [Leptos](https://leptos.dev) (SSR + hydration) et
[Axum](https://github.com/tokio-rs/axum).

- Formulaire d'inscription (prénom·s, nom·s, email, téléphone optionnel,
  numéro étudiant optionnel) avec envoi d'un email de vérification via
  SMTP, configuré dans `config.toml`.
- Emploi du temps hebdomadaire : vide pour l'instant (voir
  `src/components/weekly_schedule.rs`), affiché statiquement mais écrit
  pour être facilement branché sur la base de données plus tard.

## Démarrage rapide (NixOS)

```console
$ nix develop
$ cp config.example.toml config.toml
$ $EDITOR config.toml   # renseigne au moins les identifiants SMTP
$ cargo leptos watch
```

Le site tourne alors sur <http://127.0.0.1:3000> avec rechargement à
chaud. Le fichier `flake.nix` fournit un Rust stable récent (>= 1.88,
requis par Leptos 0.8) avec la cible `wasm32-unknown-unknown`, ainsi que
`cargo-leptos`. Si `cargo-leptos` n'est pas disponible dans ton canal
nixpkgs, installe-le dans le dev shell avec :

```console
$ cargo install cargo-leptos --locked
```

Sans Nix, il faut : Rust stable ≥ 1.88 + la cible `wasm32-unknown-unknown`
(`rustup target add wasm32-unknown-unknown`), et `cargo-leptos`
(`cargo install cargo-leptos --locked`).

## Base de données

SQLite, via `sqlx`. Le fichier indiqué par `[database].path` dans
`config.toml` (`atacc.db` par défaut) est créé et migré automatiquement au
démarrage — aucune commande à lancer à la main. Le schéma vit dans
`migrations/`.

## Ce qui manque encore

- **Le logo** : voir `public/README.md`. Le header pointe vers
  `/atacc_logo.svg`, à déposer dans `public/`.
- **Les créneaux de tutorat** : `weekly_schedule.rs` est volontairement
  statique/vide (c'est ce qui a été demandé). Le composant est structuré
  pour qu'il suffise de remplacer le corps de `creneaux_actuels()` par un
  appel à une fonction serveur qui lit la base, sans toucher au reste du
  composant.
- **Déploiement** : le document fourni mentionne un déplacement du site
  courant vers `annal.atacc.org` — ce dépôt ne présume rien sur le nom de
  domaine final, `public_url` dans `config.toml` est le seul endroit à
  ajuster (c'est la valeur utilisée dans le lien envoyé par email).

## Pépins déjà rencontrés (et corrigés)

- **`lettre` refusait de compiler** avec l'erreur *"built with the `tokio1`
  and the `native-tls` features, but `tokio1-native-tls` hasn't been turned
  on"* : `lettre` active `native-tls` par défaut, ce qui entrait en
  conflit avec `tokio1-rustls-tls`. Corrigé en passant
  `default-features = false` sur `lettre` dans `Cargo.toml`.
- **`wasm-bindgen is required but was not found`** au moment du
  `cargo leptos watch` : il manque l'outil `wasm-bindgen-cli` (distinct de
  la crate `wasm-bindgen`), et sa version doit correspondre **exactement**
  à celle résolue dans `Cargo.lock` pour la crate `wasm-bindgen`
  (`wasm-bindgen` ne suit pas semver — même un écart de patch casse le
  build). J'ai d'abord tenté un `pkgs.wasm-bindgen-cli.override { version
  = ...; }` dans `flake.nix`, mais nixpkgs a restructuré ce paquet en
  dérivations par version figées (`wasm-bindgen-cli_0_2_127`, etc.) qui
  n'acceptent plus cet override — je l'ai retiré. La solution qui marche
  dans tous les cas, à lancer dans le dev shell :

  ```console
  $ grep -A1 'name = "wasm-bindgen"' Cargo.lock   # trouve la version exacte
  $ cargo install wasm-bindgen-cli --version <cette-version> --locked
  ```

## Notes techniques / choix faits

Ce projet a été généré début septembre 2026. Les APIs de Leptos ont
beaucoup bougé entre les versions 0.6/0.7/0.8 (renderer "tachys", signaux,
imports...) ; ce code a été écrit et vérifié contre la documentation et
les templates officiels **actuels** (`leptos-rs/start-axum`, Leptos
0.8.x, Axum 0.8.x) plutôt que contre des exemples plus anciens qui
traînent sur le web. Quelques choix à connaître :

- **`sqlx` est épinglé sur la branche `0.7`** (`version = "0.7"` dans
  `Cargo.toml`) alors que `0.9` est sorti entre-temps. Je n'ai pas pu
  compiler ce projet dans mon environnement d'exécution (pas de
  toolchain Rust assez récent) pour vérifier une mise à jour vers 0.9 ;
  0.7 est une API que je connais avec confiance. Tu peux tenter de
  bumper la version toi-même (`cargo update -p sqlx`), en gardant un œil
  sur le changelog de sqlx.
- Les requêtes utilisent l'API **runtime** de sqlx (`sqlx::query`,
  `sqlx::query_as` + `#[derive(FromRow)]`) plutôt que les macros
  `query!`/`query_as!`, qui exigeraient une base de données déjà
  initialisée (ou un cache `.sqlx`) au moment de la compilation. Ça
  évite un piège classique de mise en route.
- Pas de feature `nightly` de Leptos : les signaux s'utilisent avec
  `.get()`/`.set()` plutôt que la syntaxe d'appel `signal()`. Ça reste
  du Rust stable, plus simple à outiller avec Nix.
- SMTP et la connexion sqlx utilisent toutes les deux **rustls** (pas
  OpenSSL natif), donc pas de dépendance système à `openssl-dev` a
  priori — `flake.nix` l'inclut quand même par précaution.
- **Je n'ai pas pu compiler ce code** dans le bac à sable où il a été
  généré (pas de toolchain Rust ≥ 1.88 disponible). J'ai vérifié chaque
  bout d'API contre la doc/les sources officielles à jour plutôt que de
  me fier à ma mémoire, mais il est possible qu'il reste une erreur de
  compilation mineure (import manquant, typo) à corriger au premier
  `cargo leptos watch`. Dis-moi ce que le compilateur renvoie si besoin.
