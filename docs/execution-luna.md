# Guide d'exécution autonome — agents Luna

## 1. Finalité

Implémenter une tâche bornée du [plan](superpowers/plans/2026-10-17-homebridge-rust.md), sans redécouvrir l'architecture ni élargir son périmètre. Le serveur cible est un QNAP TS-251+ avec 2 Go, la box est une TaHoma V2 et les usages comprennent io-homecontrol, RTS, garage, chauffage et neuf actions à qualifier.

Luna convient aux mappings à partir de fixtures, aux routes API déjà spécifiées, aux écrans, aux tests et au packaging isolé. Les décisions HAP, la cryptographie, les essais physiques du garage et la migration de production nécessitent une revue humaine compétente. L'agent ne choisit pas seul de retirer une protection pour faire passer un test.

## 2. Contexte à retrouver pour chaque ticket

Ne pas copier tout l'historique du chat. L'agent retrouve dans le dépôt :

1. L'ID du ticket et son texte intégral dans le plan ; le commit de départ se vérifie dans Git.
2. `docs/architecture.md`, `docs/contracts.md`, ce guide et la section utile de l'inventaire anonymisé.
3. Les fichiers possédés par la tâche et les exports des dépendances déjà fusionnées.
4. Les fixtures et les tests du composant, plus les consommateurs de l'interface modifiée.
5. La commande de vérification et les preuves attendues, notamment si un essai matériel reste à faire.

Les tâches se lancent depuis la racine du dépôt. Vérifier les instructions `AGENTS.md` présentes et les règles de l'environnement. Sur le poste du propriétaire, les commandes shell sont préfixées avec `rtk` conformément à son fichier RTK. En CI, utiliser les commandes natives sans installer RTK comme dépendance produit.

Pour toute question de bibliothèque/API/CLI : commencer par Context7 `resolve-library-id`, choisir la documentation officielle pertinente, puis `query-docs` avec la question complète. En absence de résultat, utiliser les sources officielles et signaler la limite. Ne jamais inclure les identifiants domestiques dans une requête externe.

## 3. Prompt et règle de session

Le [prompt de démarrage](prompts/start-ticket.md) est la source unique à copier. Remplacer uniquement `PXX` par l'ID choisi ; commencer par P02. L'agent déduit les fichiers, dépendances et fixtures depuis le ticket et vérifie ce qui existe réellement.

Le propriétaire demande **un ticket par session**. La session travaille en autonomie sur ce ticket, crée ou actualise sa PR et vérifie la CI, puis s'arrête. Pas de passage automatique au ticket suivant, de délégation ou de création de chat supplémentaire sans demande explicite.

Lire [AGENTS.md](../AGENTS.md), [l'état courant](project-state.md) et les [instructions GitHub Actions](../.github/instructions/github-actions-memory.instructions.md). Le modèle Luna est choisi parmi ceux effectivement disponibles dans l'environnement par le propriétaire, sans référence de modèle ancienne imposée par le dépôt.

## 4. Autonomie et limites concrètes

### L'agent peut faire sans interruption

- Lire et modifier les fichiers de son ticket, écrire des tests et compléter les exemples fictifs.
- Consulter documentation et code source officiels ; réutiliser la pile choisie.
- Construire le produit localement ou en CI, exécuter des serveurs simulés sur loopback.
- Créer sa branche et ses commits dans un espace isolé si l'environnement l'autorise.
- Corriger les appelants et tests qu'une modification autorisée affecte ; expliquer toute extension de fichiers.
- Mesurer le simulateur, détecter une fuite, proposer la correction minimale et la vérifier.

### L'agent prépare mais ne fait pas de sa propre initiative

- Lire des sauvegardes ou configurations privées hors du contexte explicitement fourni.
- Activer le mode développeur, créer/révoquer un token ou toucher à un appairage existant.
- Envoyer une commande sur un volet, un chauffage, un scénario ou une porte de garage.
- Installer un paquet sur le NAS de production, redémarrer un service existant ou désactiver un plugin.
- Déplacer les identités HomeKit, réinitialiser un pont ou restaurer des données réelles.
- Publier des captures, noms de pièces/personnes, adresses internes, PIN, clés, logs bruts ou inventaires détaillés dans GitHub.

Ces limites concernent une mission d'implémentation standard. Une instruction explicite du propriétaire pour une opération précise constitue une autorisation à prendre en compte ; elle ne doit pas être redemandée mécaniquement. Les opérations physiques du garage suivent la procédure constructeur et une présence humaine adaptée.

## 5. Contrats et partage de fichiers

- Les types exacts du contrat sont la source commune. Vérifier tous leurs appelants avant modification.
- Une modification d'API inclut dans la même PR OpenAPI, client Web, fixtures, projection HAP et tests concernés.
- `Cargo.lock`, les exports `lib.rs`, le dispatcher de mapping et les contrats ne sont pas modifiés concurremment par plusieurs agents.
- Proposer les changements transversaux au référent, qui réserve les fichiers ou séquence les tickets.
- Intégrer une PR à la fois et exécuter la CI sur la combinaison des changements.
- Si un ticket commence à nécessiter une plateforme de plugins, un bus réseau ou un nouveau protocole, revenir au besoin inventorié avant de coder.

Pas de génération de « support universel » par familles de noms de produits. Une capacité n'est ajoutée qu'avec commande déclarée, données de test, sortie UI/HomeKit définie et procédure de validation réelle.

## 6. Critères de clôture d'une tâche

- Comportement exact du ticket démontré par une preuve reproductible.
- Test négatif pertinent passé : erreurs, indisponibilité, limites ou absence de capacité selon le ticket.
- Aucune régression connue des appelants ; contrôles de format, compilation et tests requis réussis.
- Aucun secret ou donnée réelle ajouté ; aucun appareil commandé par un test automatique.
- Documentation et matrice de couverture à jour ; état « simulé », « matériel validé » ou « legacy » distinct.
- Diff limité, absence de code inutilisé et de dépendance ajoutée pour une simple opération standard.
- Revue terminée avant fusion, avec attention spécifique aux hypothèses matérielles.

## 7. Compte rendu attendu

Le rapport est enregistré dans `docs/sessions/PXX.md` et résumé dans `docs/project-state.md`. Il fait partie de la PR du ticket. Une PR prête reste `ready_for_review` jusqu'à intégration vérifiée ; elle n'est pas assimilée à un ticket terminé sur `main`.

```text
Ticket : Pxx
Résultat : comportement obtenu, en une phrase.
Commit : identifiant.
Fichiers modifiés : liste courte.
Interfaces produites/modifiées : noms et effet sur les consommateurs.
Vérifications : commande, résultat et durée si utile.
Matériel réellement testé : aucun / modèle et scénario autorisé.
Compatibilité : native simulée / native validée / legacy / bloquée.
Limites : hypothèse restante, preuve manquante et ticket qui la possède.
Prochaine dépendance disponible : ticket/API désormais exploitable.
```

Une réponse « ça compile » n'est pas une preuve de compatibilité HomeKit ou Somfy. Une capture de la page Homebridge ne prouve pas qu'un volet s'est déplacé. Un retour de commande accepté ne prouve pas que le garage est fermé.
