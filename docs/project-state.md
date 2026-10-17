# Mémoire et état courant du projet

Dernière mise à jour : 17 octobre 2026. Ce fichier est le point de reprise des sessions ; le code, les commits et la CI priment si son état n'a pas encore été réconcilié après une fusion.

## Décisions du propriétaire

- Dépôt GitHub **public** sous `magic5644` : [magic5644/homebridge-rust](https://github.com/magic5644/homebridge-rust).
- **Un ticket par session**, modèle choisi par le propriétaire ; Luna est prévu pour les tâches bornées. Pas d'enchaînement ou de délégation automatique.
- Chaque session lit les instructions du dépôt, vérifie ses dépendances et laisse un rapport versionné. Les détails GitHub Actions sont mémorisés dans [les instructions CI](../.github/instructions/github-actions-memory.instructions.md).
- Déploiement final sur QNAP TS-251+, Celeron J1900, 2 Go ; Docker et installation native QPKG.
- TaHoma V2 et équipements io-homecontrol/RTS. Les usages connus incluent volets/stores, garage, chauffage et actions virtuelles.
- Migration progressive avec maintien du fonctionnement actuel ; aucun contrôle physique ou changement de production dans une session de code standard.

## Où trouver l'information

| Besoin | Source |
|---|---|
| Instructions chargées au début du travail | [AGENTS.md](../AGENTS.md) |
| Périmètre et choix d'architecture | [Architecture](architecture.md) |
| Types, API et invariants | [Contrats](contracts.md) |
| Tickets et dépendances | [Plan P01–P38](superpowers/plans/2026-10-17-homebridge-rust.md) |
| Commande de départ d'une session | [Prompt réutilisable](prompts/start-ticket.md) |
| Méthode et limites Luna | [Guide d'exécution](execution-luna.md) |
| Matériel et informations manquantes | [Inventaire](inventory-template.md) |
| Sources et preuves de recherche | [Recherche](research.md) |
| Règles CI/publication | [Mémoire GitHub Actions](../.github/instructions/github-actions-memory.instructions.md) |
| Résultat d'une session de ticket | `docs/sessions/PXX.md`, créé par la session concernée |

Les Pxx sont actuellement des identifiants du plan Markdown. Ne pas inventer de correspondance avec `#2`, `#3`, etc. Un lien d'issue/PR est ajouté seulement après création ou lecture vérifiée.

## État de livraison vérifié

| Élément | État |
|---|---|
| P01 — dépôt et conception | `done` ; conception publiée sur `main`, référence initiale `dec76c0` |
| Instructions et mémoire de session | Ajoutées dans le dépôt ; modification documentaire, hors implémentation P02 |
| P02 — workspace et CI Rust | `not_started` ; **prochain ticket à lancer** |
| P03 — inventaire détaillé | Attend P02 ; lecture visuelle déjà réalisée, inventaire TaHoma authentifié restant |
| P04 — décision HAP | Attend P02 ; aucun backend natif encore approuvé |
| P05 et suivants | Non commencés ; respecter les dépendances du plan |
| Application / tests produit / CI | Pas encore créés ; aucun gain mémoire ni compatibilité native démontré |

Le harnais documentaire est en place ; le harnais exécutable est le livrable P02 et sera enrichi par les tickets de simulateur et de tests. Ne pas déclarer P02 réalisé parce que les commandes et workflows sont décrits.

## GitHub — observations en lecture seule

Vérifiées via l'API GitHub le 17 octobre 2026, avant création de la mémoire :

- Propriétaire/nom : `magic5644/homebridge-rust` ; public ; branche par défaut `main`.
- Issues et Projects activés ; aucun board ou découpage en issues configuré par ces sessions.
- GitHub Actions activé ; politique retournée : `allowed_actions=all`, `sha_pinning_required=false`. Les fichiers de ce projet imposent leur propre épinglage ; ils ne modifient pas cette politique distante.
- Aucun ruleset retourné par l'API. Les protections classiques de branche n'ont pas été auditées dans cette vérification.
- Aucun workflow exécutable dans le dépôt au moment de cette mise à jour. CI prévue : `.github/workflows/check.yml`, job affiché `Rust checks`, à réaliser et exécuter au P02.
- Aucun paramètre distant de permissions/protection/runner n'a été modifié par la mémorisation.

Authentification : la session GitHub CLI a fonctionné pour ce compte. Un push via le gestionnaire de credentials système a attendu ; le helper GitHub CLI utilisé ponctuellement a permis la synchronisation. Si cela revient, diagnostiquer le helper par commande sans exposer un token ni modifier la configuration globale.

## Faits techniques acquis

- Observation UI : Homebridge 2.4.0, UI 5.29.0, Node 24.21.0, homebridge-tahoma 2.2.64 en child bridge ; conteneur Ubuntu 22.04.5 x64.
- 23 éléments visibles : 11 coverings, 2 thermostats, 1 garage, 9 Switch ; types UI à qualifier par capacités réelles.
- API développeur TaHoma découverte ; HTTPS avec CA et nom DNS validés ; lecture publique `apiVersion` réussie, protocole `2026.4.5-7`.
- Aucun token TaHoma utilisé pour ce test, aucune commande physique envoyée et aucune modification de Homebridge/NAS effectuée.

Coordonnées internes, PIN, tokens et noms d'appareils ne sont pas stockés dans cette mémoire publique.

## Fin de chaque prochaine session

1. Mettre à jour uniquement l'état du ticket traité et les faits devenus vérifiés.
2. Ajouter/actualiser `docs/sessions/PXX.md`, avec branche/PR, commit testé, commandes et résultats CI.
3. Conserver les limites : test simulé ≠ preuve matérielle ; PR prête ≠ fusion sur `main` ; workflow écrit ≠ CI réussie.
4. Indiquer le ticket suivant éligible selon le plan, sans le commencer. P03 et P04 deviennent tous deux possibles après fusion et validation de P02 ; le propriétaire choisit leur ordre.
