# Inventaire et matrice de couverture

Compléter une copie dans `private/inventory.md` ou un emplacement privé. Le dépôt public ne contient que les profils techniques anonymisés, les comptes d'éléments et leur niveau de couverture. Aucun mot de passe, token, PIN de box ou d'appairage n'est demandé dans ce document.

## 1. Matériel confirmé

| Élément | Valeur connue | À relever |
|---|---|---|
| NAS | QNAP TS-251+ | Version QTS et version Container Station |
| CPU/RAM | Intel Celeron J1900, 2 Go déclarés par le propriétaire | Noyau, jeu d'instructions et compatibilité binaire sur l'hôte |
| Installation actuelle | Docker ; UI rapporte Ubuntu 22.04.5, x64 | Image et digest, volumes, ports, mode réseau, limites de ressources |
| Homebridge | 2.4.0 | Sauvegarde et configuration utile expurgée |
| UI | 5.29.0 | Paramètres d'accès et proxy sans secrets |
| Node | 24.21.0 | Processus requis et PSS/RSS de chacun |
| Plugin | homebridge-tahoma 2.2.64, child bridge | Service local/cloud, options par profil, groupes exposés |
| Box | TaHoma V2 | Modèles des moteurs et capacités exactes via setup |
| API locale | API développeur annoncée ; protocole 2026.4.5-7 ; TLS et GET apiVersion validés | Authentification et setup avec token local |
| Contrôleurs | Apple Maison/Siri supposés par le projet Homebridge | Versions, concentrateur, automatisations et Matter réellement utilisés |

## 2. Éléments visibles — lecture seule du 17 octobre 2026

Les noms domestiques ne sont pas repris. Les numéros suivants sont des repères d'inventaire, pas les UUID réels du système.

| Groupe | Nombre | Ce qui est confirmé | Ce qui reste à qualifier | Ticket |
|---|---:|---|---|---|
| Coverings | 11 | Présents dans l'UI ; le propriétaire confirme io et RTS | Répartition io/RTS, volet/store, commandes, position/tilt et calibration | P12/P13 |
| Thermostats | 2 | Présents dans l'UI | Mesure réelle, fil pilote éventuel, consignes et modes | P34 |
| Garage | 1 | Présent comme Garage Door Opener | Radio, modèle, impulsion ou commandes distinctes, état et obstacle | P33 |
| Switch | 9 | Présents comme interrupteurs | Scénarios, groupes ou appareils ; effets et dépendances | P35 |
| Total | 23 | Éléments d'interface, pas nécessairement 23 appareils physiques | Autres usages non visibles et dépendances de scènes | P36 |

Ne pas déduire de la valeur affichée une mesure physique vérifiée. Les commandes n'ont pas été actionnées pendant cet inventaire.

## 3. Une ligne par appareil ou action, en copie privée

| Repère anonyme | Famille/modèle | io/RTS/autre | Commandes utilisées | États réellement disponibles | Limites/calibration | Automatisations dépendantes | Propriétaire actuel | Cible | Validation |
|---|---|---|---|---|---|---|---|---|---|
| covering-01 | À relever | À relever | Ouvrir/fermer/stop/position selon usage | À relever dans setup puis comparer physiquement | À relever | À relever | legacy | observe puis native | non validé |
| heating-01 | À relever | À relever | À relever | Température réelle ou absente | Bornes et modes | À relever | legacy | native après P34 | non validé |
| garage-01 | À relever | À relever | Déterministes ou impulsion | Position/fermeture/obstacle, avec preuve | Procédure constructeur | À relever | legacy | mode validé P33 | non validé |
| action-01 | Scénario/groupe/équipement à qualifier | À relever | Effets exacts | Résultat/échec partiel si disponible | Dépendances | À relever | legacy | P35 | non validé |

Étendre à tous les éléments, sans réduire plusieurs commandes différentes à une ligne « Somfy supporté ».

## 4. Informations garage

- Référence du moteur et type de radio, retrouvés sur documentation/facture ou setup sans démontage.
- Commandes exposées et effet : ouvrir, fermer, stop ou impulsion cyclique.
- Retour d'état : mesure de fin de course, position, état calculé par plugin ou absence de mesure.
- Détection d'obstacle : capteur réel, remontée accessible, comportement constructeur.
- Fonctionnement en cas de commande perdue/doublée, action de la télécommande et mouvement en cours.
- Mode HomeKit aujourd'hui, automatismes critiques et procédure de contrôle en présence du propriétaire.

## 5. Informations chauffage et actions

- Chaque thermostat : capteur réellement présent, modèle, unités, consigne, modes, programme, durée de dérogation et retour d'état.
- Chaque Switch : déclenchement momentané ou état maintenu ; scénario fabricant ou commande directe ; tous les appareils touchés, notamment garage et chauffage.
- Les horaires ou conditions stockés dans TaHoma restent dans TaHoma tant qu'une équivalence explicite n'est pas établie.

## 6. Baseline de ressources

Noter date, durée, version, nombre d'appareils, état des connexions et fonctions actives. Pour chaque processus requis : RSS, PSS si disponible, CPU et éventuels processus enfants ; pour Docker : cgroup courant/pic, séparément. Conserver le brut en privé et publier uniquement un résumé technique.

Les indicateurs total/disponible du widget mémoire Homebridge ne mesurent pas à eux seuls la consommation de Homebridge. Une capture ponctuelle de charge CPU n'est pas une baseline.

## 7. Définition des statuts

| Statut | Signification |
|---|---|
| legacy | Fonction conservée dans Homebridge ; intégration native non livrée |
| fixture-tested | Conversion testée sur données anonymisées ; matériel non validé |
| observe | Lecture Rust active, commandes/publication Rust bloquées |
| native-pilot | Contrôle validé en pilote, période de stabilité en cours |
| native-validated | Matériel, UI, HomeKit, pannes et rollback validés |
| gap | Fonction non couverte ; maintien legacy obligatoire ou décision requise |

La suppression de Homebridge exige zéro usage `legacy` ou `gap`, et le retrait de Node.js exige aussi un transport HAP natif validé.
