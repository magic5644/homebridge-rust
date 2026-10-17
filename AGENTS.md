# Instructions du projet

## Démarrer chaque session

1. Lire [l'état courant](docs/project-state.md), le [guide de session](docs/prompts/start-ticket.md) et le ticket demandé dans le [plan](docs/superpowers/plans/2026-10-17-homebridge-rust.md).
2. Lire les parties utiles de [l'architecture](docs/architecture.md), des [contrats](docs/contracts.md) et du [guide Luna](docs/execution-luna.md), puis les fichiers concernés et leurs appelants.
3. Vérifier l'état Git, la branche, les changements existants, les PR et les dépendances du ticket. Une dépendance seulement décrite ou dans une PR non fusionnée n'est pas disponible sur `main`.
4. Exécuter **un seul ticket par session**. Ne pas enchaîner le suivant, lancer de sous-agents ou créer d'autres chats sans demande explicite. Le modèle de la session est choisi par le propriétaire ; aucune sélection automatique de modèle.

Les instructions explicites du propriétaire dans la session priment sur les conventions de ce dépôt. Une validation déjà donnée ne doit pas être redemandée mécaniquement.

## Projet et décisions à préserver

- Dépôt public : `https://github.com/magic5644/homebridge-rust`, propriétaire `magic5644`, branche de référence `main`.
- Cible : QNAP TS-251+, Celeron J1900, 2 Go ; TaHoma V2 ; volets io-homecontrol et RTS, garage, deux thermostats et neuf actions à qualifier.
- Quatre packages Rust prévus : `bridge-core`, `integration-tahoma`, `transport-homekit`, `bridge-server`. UI Svelte/TypeScript statique au P17.
- Distribution Docker amd64 et paquet QNAP `.qpkg` ; aucune compilation sur le NAS. Le choix du transport HAP natif reste conditionné au P04.
- La compatibilité est prouvée par appareil et usage. Garder Homebridge pour les fonctions non remplacées ; son maintien ne constitue pas une intégration Rust livrée.
- La mémoire durable vit dans ces fichiers versionnés, pas seulement dans le chat ou une mémoire personnelle d'agent. L'état d'implémentation réel se vérifie dans Git et la CI.

## Exécution du ticket

- Travailler sur une branche dédiée `feat/pXX-sujet` ; si une branche/PR du ticket existe, l'examiner avant d'en créer une autre. Préserver les modifications locales du propriétaire.
- Réutiliser les interfaces du contrat et les conventions en place. Implémenter le minimum complet, sans fonctionnalités futures ou abstraction supplémentaire.
- Pour une logique non triviale, écrire un test de comportement, observer son échec pertinent, puis implémenter et vérifier. Une correction modifie aussi tous les appelants et fixtures affectés.
- Un changement de contrat commun inclut ses consommateurs, sa documentation et ses tests dans le même ticket. Signaler l'impact ; ne pas réserver la correction à une autre session.
- Les tâches ordinaires autorisées avancent sans demande de confirmation. Si un prérequis indispensable manque, préciser lequel et terminer les travaux indépendants sans inventer une preuve.
- Avant de conclure, vérifier le diff et les contrôles requis ; indiquer ceux qui n'ont pas été exécutés et pourquoi.

## Documentation technique et commandes

- Pour bibliothèques, frameworks, SDK, API, CLI et services cloud : utiliser Context7, d'abord `resolve-library-id` sauf ID exact fourni, puis `query-docs` avec la question complète. Choisir la source officielle pertinente ; versions explicites quand connues. Sans documentation accessible, signaler la limite plutôt qu'inventer une API.
- Sur le poste du propriétaire, lire le fichier RTK de l'environnement s'il existe et préfixer les commandes shell avec `rtk` (`rtk proxy` pour une commande non prise en charge). En CI, utiliser les outils natifs ; RTK n'est pas une dépendance produit.
- Utiliser `rg` pour la recherche. Ne pas modifier les réglages globaux Git/Codex pour résoudre un problème propre au dépôt.
- Les commandes Rust ci-dessous deviennent exécutables avec P02 ; ne pas annoncer leur réussite avant création du workspace et exécution réelle :

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build -p bridge-server --release --locked
```

P02 crée `scripts/check.sh`, point d'entrée de ces quatre contrôles, utilisé localement et en CI. Les scripts frontend ne sont requis qu'à partir du ticket qui crée `web/package.json`.

## GitHub, CI et livraisons

Lire [.github/instructions/github-actions-memory.instructions.md](.github/instructions/github-actions-memory.instructions.md) pour tout changement de workflow, dépendance de build, packaging ou publication.

- Vérifier le dépôt et le compte avant une écriture distante ; utiliser la session GitHub existante, sans copier de token dans un fichier ou une commande.
- Un ticket de code donne une branche, un commit limité et une PR vers `main`, avec tests, limites et lien vers le ticket du plan. Ne pas assimiler un ID Pxx à un numéro d'issue GitHub.
- Les prompts de démarrage autorisent commit, push de la branche et création/mise à jour de sa PR. Aucune fusion, publication de release ou installation de production automatique.
- Attendre le résultat CI du commit poussé ; inspecter les échecs relevant du ticket et les corriger. Si l'exécution distante est inaccessible ou encore en attente, le dire explicitement et ne pas déclarer le ticket terminé.
- Aucun workflow ne déploie sur le NAS ni n'utilise son réseau domestique. Les tests matériel restent séparés des PR publiques.

## Données et matériel

- Fixtures fictives par défaut. Données privées dans `private/`, secrets et état d'exécution hors Git ; ne pas ajouter d'exceptions à `.gitignore` pour les publier.
- Les modes `observe` et `legacy` interdisent commandes et publication HomeKit depuis Rust.
- Aucun essai automatique sur des équipements réels, aucune restauration de production, aucun reset d'appairage ou changement de plugin sans autorisation précise.
- Une commande physique incertaine n'est pas rejouée. Une position RTS estimée ou un état de garage inconnu n'est pas une mesure confirmée.
- Les secrets, PIN, cookies, identifiants de passerelle, noms domestiques et captures privées ne figurent jamais dans GitHub, les logs CI, les rapports publics ou les requêtes de documentation.

## Fin de session et mémoire

- Écrire `docs/sessions/PXX.md` : but, branche/PR, commit d'implémentation testé, changements, commandes et résultats, état CI, limites, dépendances désormais disponibles. Éviter un hash auto-référent : citer le commit testé avant ajout du rapport ou le dernier commit d'implémentation.
- Actualiser `docs/project-state.md` avec le résultat et le point de reprise. Mettre à jour les cases du ticket uniquement sur preuve.
- Distinguer `in_progress`, `implemented`, `ci_pending`, `ready_for_review`, `blocked`, `done`. `done` exige les critères du ticket et l'intégration vérifiée sur `main` ; tests matériel/humains restent requis quand le ticket les exige.
- Après fusion par le propriétaire, la session suivante vérifie le commit et la CI, puis réconcilie l'état si le rapport précédent indiquait encore `ready_for_review`.
- Conclure par résultat, PR/commit, vérifications et limites. S'arrêter après ce ticket ; ne jamais démarrer le prochain automatiquement.
