# Démarrer une session de ticket

Ouvrir une nouvelle session dans le projet `Homebridge-rust`, choisir le modèle souhaité puis coller le prompt ci-dessous. Remplacer uniquement `PXX` par l'identifiant voulu. Le premier ticket de développement est **P02**.

```text
Implémente uniquement le ticket PXX de ce dépôt.

Lis AGENTS.md, docs/project-state.md et la section PXX de
docs/superpowers/plans/2026-10-17-homebridge-rust.md.
Lis ensuite les contrats, l'architecture et les fichiers cités par ce ticket.
Pour la CI et GitHub, applique aussi
.github/instructions/github-actions-memory.instructions.md.

Vérifie l'état Git, les PR et que les prérequis sont réellement disponibles.
Travaille sur une branche dédiée au ticket, en préservant mes changements.
Cette session traite un seul ticket, sans délégation automatique.
Avance en autonomie sur toutes les étapes ordinaires autorisées.

Implémente le minimum complet, avec tests pertinents et vérifications réelles.
N'utilise que des données fictives pour les tests et ne commande aucun
équipement réel. Ne lance ni installation de production, ni release.

Une fois les contrôles locaux réussis, committe les fichiers du ticket,
pousse sa branche et crée ou actualise une PR vers main, sans la fusionner.
Vérifie la CI du dernier commit poussé et corrige ses échecs liés au ticket.
Si une preuve ou un prérequis indispensable est inaccessible, explique-le
précisément ; ne marque pas une validation non exécutée comme réussie.

Mets à jour docs/project-state.md, les cases prouvées du plan et
docs/sessions/PXX.md ; inclus ces documents dans la PR puis vérifie aussi
le dernier commit. Cite le commit d'implémentation testé dans le rapport
sans chercher à rendre le rapport auto-référent.

Termine avec résultat, lien de PR, contrôles effectués et limites restantes.
Arrête-toi après ce ticket. Ne démarre pas le suivant.
```

## Particularités de P02

P02 construit le premier harnais exécutable : quatre packages Rust minimaux, toolchain et lockfile figés, CLI minimale, tests CLI, `scripts/check.sh` et workflow `.github/workflows/check.yml` exécuté sur GitHub.

Les instructions et la mémoire sont déjà présentes : les compléter si nécessaire, sans les reconstruire ni implémenter les fonctionnalités Somfy/HAP des tickets suivants. Le frontend complet appartient au P17.

Le choix du ticket P02 autorise son implémentation dans le périmètre de l'architecture proposée ; il n'est pas nécessaire de redemander une approbation pour le workspace minimal. Les décisions HAP, tests matériels et bascules restent aux lots qui les portent.

Pour un ticket ultérieur, vérifier la fusion de ses dépendances. P03 exige des données et accès locaux ; P04 exige une preuve HomeKit distincte. Un manque d'accès ne se corrige pas en inventant des fixtures prétendument capturées ou des validations matérielles.
