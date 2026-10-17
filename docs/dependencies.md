# Versions et licences — P02

Versions vérifiées le 17 octobre 2026. Le code du projet est sous [licence MIT](../LICENSE).
Aucun code Homebridge, HAP ou fabricant n'est repris dans ce socle.

## Choix minimaux

- Rust **1.99.1**, édition **2024**, resolver **3** : version stable vérifiée dans le
  [manifeste officiel](https://static.rust-lang.org/dist/channel-rust-1.99.1.toml).
  `rust-toolchain.toml` fixe le canal, le profil minimal, rustfmt et clippy.
  Le compilateur Rust est distribué sous MIT OU Apache-2.0.
- clap **4.6.7**, feature `derive`, MIT OU Apache-2.0 : seule dépendance directe,
  limitée au binaire CLI. Version exacte dans `Cargo.toml`, transitives et checksums
  dans `Cargo.lock`. Aucun runtime async, HTTP, stockage ou backend HAP au P02.
- Les quatre packages locaux sont en **0.1.0**, non publiables sur crates.io.
- CI : runner **ubuntu-24.04**, `actions/checkout` **v7.0.1** (MIT), SHA
  `3d3c42e5aac5ba805825da76410c181273ba90b1`, vérifié dans le
  [dépôt officiel](https://github.com/actions/checkout/releases/tag/v7.0.1).
  Runtime Node 24 fourni par l'action ; runner GitHub hébergé compatible.
  rustup préinstallé sélectionne la toolchain depuis le fichier versionné ;
  `rustup toolchain install` sans argument l’installe si elle manque.

Documentation consultée via Context7 (`resolve-library-id`, puis `query-docs`) :
[clap derive](https://docs.rs/clap/4.6.7/clap/_derive/_tutorial/index.html),
[workspaces Cargo](https://doc.rust-lang.org/cargo/reference/workspaces.html),
[checkout](https://github.com/actions/checkout),
[rustup](https://rust-lang.github.io/rustup/overrides.html),
[concurrence Actions](https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/control-deployments).
Les versions publiées ont aussi été vérifiées sur crates.io, static.rust-lang.org et
l'API GitHub ; les exemples Context7 seuls ne prouvent pas la dernière version.

## Dépendances verrouillées

Table issue de `cargo metadata --format-version 1 --locked`, toutes cibles comprises.
Le champ licence est l'expression SPDX déclarée par chaque composant ; il ne
remplace pas son texte de licence. Les archives crates.io conservent leurs licences
et notices originales ; aucun code tiers n'est copié ou vendorié dans le dépôt.
P30 devra inclure ces textes/notices avec les artefacts redistribués et vérifier
à nouveau cette liste après ajout des dépendances des tickets suivants.

| Composant | Version exacte | Licence déclarée |
|---|---|---|
| anstream | 1.0.0 | MIT OR Apache-2.0 |
| anstyle | 1.0.14 | MIT OR Apache-2.0 |
| anstyle-parse | 1.0.0 | MIT OR Apache-2.0 |
| anstyle-query | 1.1.5 | MIT OR Apache-2.0 |
| anstyle-wincon | 3.0.11 | MIT OR Apache-2.0 |
| clap | 4.6.7 | MIT OR Apache-2.0 |
| clap_builder | 4.6.7 | MIT OR Apache-2.0 |
| clap_derive | 4.6.7 | MIT OR Apache-2.0 |
| clap_lex | 1.1.1 | MIT OR Apache-2.0 |
| colorchoice | 1.0.5 | MIT OR Apache-2.0 |
| heck | 0.5.0 | MIT OR Apache-2.0 |
| is_terminal_polyfill | 1.70.2 | MIT OR Apache-2.0 |
| once_cell_polyfill | 1.70.2 | MIT OR Apache-2.0 |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 |
| quote | 1.0.47 | MIT OR Apache-2.0 |
| strsim | 0.11.1 | MIT |
| syn | 3.0.9 | MIT OR Apache-2.0 |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 |
| utf8parse | 0.2.2 | Apache-2.0 OR MIT |
| windows-link | 0.2.1 | MIT OR Apache-2.0 |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 |
