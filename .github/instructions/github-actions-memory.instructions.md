---
description: Mémoire du projet pour les contrôles GitHub Actions et la publication des artefacts.
applyTo: ".github/workflows/**/*.yml,.github/workflows/**/*.yaml,scripts/**,packaging/**"
---

# GitHub Actions Memory

Le dépôt est public. Les validations du code et la publication d'artefacts ont des périmètres distincts. Ces consignes sont des décisions du projet ; leur présence ne signifie pas qu'un workflow existe déjà.

## CI initiale à réaliser au P02

- Fichier `.github/workflows/check.yml`, nom du workflow `CI`, job `rust` avec nom affiché stable `Rust checks`.
- Déclencheurs : `pull_request`, `push` sur `main`, `workflow_dispatch`. Exécuter aussi le contrôle sur les PR ne touchant que des documents tant que ce job doit servir de contrôle requis ; éviter un filtre de chemins qui le laisserait en attente.
- Runner GitHub hébergé Linux, version explicite compatible avec les actions sélectionnées. Aucun runner sur le NAS ni dans le réseau domestique pour les PR publiques.
- Permission de validation : `contents: read` ; aucun secret de déploiement, de compte fabricant ou de production.
- Actions tierces épinglées au SHA complet vérifié dans leur dépôt officiel ; ajouter un commentaire indiquant la version lisible. Choisir les versions/runtimes depuis la documentation courante, pas depuis un exemple ancien.
- Checkout sans persistance de credentials pour les étapes de build. Aucun accès en écriture requis pour compiler ou tester.
- Toolchain depuis `rust-toolchain.toml`, dépendances depuis `Cargo.lock` versionné ; contrôle `--locked` pour clippy, tests et build.
- Exécuter `sh scripts/check.sh`, également utilisé localement ; les quatre contrôles exacts sont définis dans `AGENTS.md`.
- Timeout initial de 20 minutes pour le job. Annuler les runs CI précédents du même workflow et de la même branche/PR quand un nouveau commit les remplace ; ne pas appliquer cette annulation à une publication en cours.
- P02 valide aussi le script par `sh -n`, son comportement en cas d'échec et une exécution Linux réelle dans Actions. Le lint YAML seul ne constitue pas une CI verte.

## Évolution des contrôles

- P17 ajoute les contrôles frontend quand le frontend existe : installation par lockfile, vérification des types, tests et build statique. Les E2E utilisent le serveur simulé.
- Les tests de contrats et packaging rejoignent la CI avec les composants concernés. La suite initiale n'annonce pas de tests Somfy/HAP inexistants.
- Un cache est optionnel : l'ajouter pour un gain mesuré, avec clé tenant compte de l'OS, de la toolchain et du lockfile. Ne jamais mettre en cache secrets, base domestique ou état HomeKit.
- Garder les noms de contrôles stables ; changer un contrôle requis implique de vérifier les règles GitHub qui l'attendent.
- Le succès d'un job sur un ancien commit ne prouve pas celui du dernier commit de la PR.

## Code de PR non fiable

- Utiliser `pull_request` pour les builds/tests ; ne pas exécuter le code de la tête d'une PR sous `pull_request_target` avec droits ou secrets privilégiés.
- Traiter titre, corps, branche et autres métadonnées de PR comme des données. Ne pas les interpoler directement dans un script shell.
- Les PR de forks ne reçoivent aucun secret de production. Ne pas changer les permissions pour contourner une étape de test manquante.
- Contrôler le contenu des logs et artefacts : pas de configuration réelle, token, PIN, noms domestiques ou sauvegarde.

Ces règles suivent la [documentation GitHub sur l'usage sécurisé](https://docs.github.com/en/actions/reference/security/secure-use) et les [risques de pull_request_target](https://docs.github.com/en/actions/reference/security/securely-using-pull_request_target), consultées via Context7 le 17 octobre 2026.

## Publication au P30

- La release est un workflow séparé de la CI. Sa réalisation appartient au P30 ; P02 ne publie ni image GHCR, ni paquet QPKG, ni release.
- Destination prévue : `ghcr.io/magic5644/homebridge-rust`. Docker amd64 prioritaire pour le QNAP ; arm64 secondaire avec niveau de test indiqué.
- Artefacts : image versionnée/digest, archive native, paquet `.qpkg`, checksums et licences. Construire pour CPU x86_64 générique, sans `target-cpu=native` du poste de compilation.
- Permissions de publication uniquement dans les jobs qui en ont besoin, par exemple `packages: write` pour GHCR et `contents: write` pour une release, jamais globalement par commodité.
- Vérifier l'accès anonyme au package si l'image doit être publique ; visibilité du dépôt et visibilité GHCR sont distinctes.
- Les profils natif et hybride restent identifiés. Une release stable attend les preuves d'acceptation prévues au plan ; aucun déploiement sur le NAS par simple push/tag.

## Réglages GitHub et mémoire

- Les valeurs observées à distance sont datées dans `docs/project-state.md`. Les revalider avant de les utiliser ; elles peuvent changer indépendamment du dépôt.
- Des consignes dans un fichier ne créent pas un contrôle obligatoire ni une protection de branche.
- Après la première CI verte, proposer les noms de contrôles à rendre requis sur `main`. Toute modification de règles ou de permissions doit correspondre à une demande explicite ; cette session de mémorisation ne les modifie pas.
- Les capacités Issues/Projects ne signifient pas que les Pxx ont été créés comme issues ; conserver les liens réels lorsqu'ils existent.
