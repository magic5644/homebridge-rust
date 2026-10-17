# Homebridge Rust

Proposition du 17 octobre 2026 pour `magic5644` : un pont domotique économe en ressources, administrable sur le Web, installable sur NAS ou avec Docker, avec intégration Somfy/TaHoma prioritaire.

**État : socle CLI/workspace P02 ; aucune intégration domotique ni migration effectuée.** Les objectifs chiffrés sont des critères à mesurer, pas des gains déjà obtenus.

## Lire le dossier

1. [Architecture et décisions](docs/architecture.md) : périmètre, alternatives, technologies, HomeKit, Somfy, installation, performances et migration.
2. [Contrats communs](docs/contracts.md) : modèle de données, interfaces Rust, API Web et règles partagées.
3. [Plan d'intégration et tickets](docs/superpowers/plans/2026-10-17-homebridge-rust.md) : ordre d'exécution, fichiers, tests et conditions de clôture de chaque tâche.
4. [Guide d'exécution Luna](docs/execution-luna.md) : contexte minimal, autonomie, limites et modèle de compte rendu.
5. [Inventaire à compléter](docs/inventory-template.md) : matériel, plugins, capacités et couverture de chaque équipement.
6. [Recherche et vérifications](docs/research.md) : sources, documentation Context7 et observations locales sans identifiants de la maison.

## Une session par ticket

Commencer par [l'état courant](docs/project-state.md), puis utiliser le [prompt de démarrage](docs/prompts/start-ticket.md) avec le ticket choisi après vérification de ses dépendances. Les [instructions agents](AGENTS.md) et la [mémoire GitHub Actions](.github/instructions/github-actions-memory.instructions.md) sont versionnées pour les sessions suivantes.

La session implémente son ticket, teste, prépare une PR et laisse un rapport ; elle ne démarre pas automatiquement le suivant. Le harnais exécutable P02 est décrit dans [son rapport](docs/sessions/P02.md).

## Orientation recommandée

Construire le serveur et les intégrations en Rust. Tester très tôt la viabilité de HomeKit natif avant de le choisir définitivement. Garder Homebridge pour les équipements qui dépendent encore de ses plugins ; prévoir un petit transport HAP-NodeJS temporaire si la pile HAP Rust ne passe pas les tests.

La compatibilité avec toute la maison se vérifie équipement par équipement. Un plugin JavaScript existant ne devient pas un plugin Rust par recompilation. Le profil entièrement Rust ne sera livré qu'après validation de HomeKit et remplacement de toutes les intégrations nécessaires.

## Dépôt et confidentialité

Dépôt **public** créé : [magic5644/homebridge-rust](https://github.com/magic5644/homebridge-rust), selon le choix du propriétaire. La publication initiale contient uniquement les documents. Le compte, la visibilité et les fichiers distants ont été vérifiés.

Ne jamais versionner de jetons, de sauvegardes Homebridge réelles, d'identifiants TaHoma, d'adresses des appareils ou de données d'appairage HomeKit. Les exemples et tests utiliseront des valeurs fictives.

## Développer le socle P02

Installer Rust **1.99.1** avec rustfmt et clippy (rustup lit `rust-toolchain.toml`),
puis lancer à la racine :

```sh
sh scripts/check.sh
cargo run -p bridge-server --locked -- --version
cargo run -p bridge-server --locked -- check --config /chemin/vers/config.toml
```

`check` lit un fichier UTF-8 sans le modifier et échoue si le chemin est absent,
inaccessible ou désigne un répertoire. Il ne valide pas encore un schéma métier,
ne démarre aucun serveur et ne contacte aucun équipement.
Les trois bibliothèques sont des socles vides pour les tickets suivants.

Versions et licences : [dépendances](docs/dependencies.md). Code sous [MIT](LICENSE).
