# Recherche et vérifications — 17 octobre 2026

## Méthode et limites

Documentation officielle et dépôts des projets consultés en lecture seule. Context7 a servi pour Axum, Svelte et Docker : appel `resolve-library-id`, choix des projets officiels, puis `query-docs` avec le besoin d'un serveur domotique Rust léger sur NAS. Les autres détails spécifiques sont recoupés avec leurs documentations primaires. Les versions exactes seront figées au démarrage du développement.

Identifiants Context7 sélectionnés : `/tokio-rs/axum`, `/sveltejs/svelte`, `/docker/docs`. Les sorties complètes de recherche ne sont pas intégrées au produit ; les liens ci-dessous permettent de retrouver les sources.

## Sources et décisions associées

| Source primaire | Utilisation dans la proposition |
|---|---|
| [Homebridge](https://github.com/homebridge/homebridge) | Moteur, écosystème de plugins, portée de la compatibilité et présence possible de Matter |
| [Homebridge UI](https://github.com/homebridge/homebridge-config-ui-x) | Fonctionnalités d'administration à couvrir progressivement |
| [HAP-NodeJS](https://github.com/homebridge/HAP-NodeJS) | Transport HomeKit de référence et repli temporaire |
| [Child Bridges](https://github.com/homebridge/homebridge/wiki/Child-Bridges) | Coexistence, isolation, limites et identités distinctes |
| [hap-rs](https://github.com/ewilken/hap-rs) | Candidat Rust à évaluer, pas dépendance déjà approuvée |
| [Commit hap-rs consulté](https://github.com/ewilken/hap-rs/commit/acf45bdb00e23c81b67b3d5116c113335dc83f16) | Dernier commit renvoyé par l'API du dépôt : 14 avril 2023 |
| [Guide développeur Somfy](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode) | Authentification locale, certificat, découverte et recommandation de suivi événementiel |
| [OpenAPI TaHoma](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode/blob/main/docs/openapi.yaml) | Routes/verbres/capacités à utiliser ; ne pas extrapoler des scénarios non documentés |
| [homebridge-tahoma](https://github.com/dubocr/homebridge-tahoma) | Comparaison des options existantes et risques de changement d'identifiants cloud/local |
| [Axum](https://docs.rs/axum/latest/axum/) | Serveur HTTP et intégration Tokio/Tower |
| [Tokio — arrêt gracieux](https://tokio.rs/tokio/topics/shutdown) | Annulation et suivi des tâches |
| [Svelte](https://svelte.dev/docs/svelte/overview) | UI compilée, sans obligation d'un serveur Node en production |
| [Vite — distribution statique](https://vite.dev/guide/static-deploy.html) | Build d'assets intégrable au serveur Rust |
| [reqwest](https://docs.rs/reqwest/latest/reqwest/) | Client async réutilisé et configuration TLS/proxy |
| [rusqlite](https://docs.rs/rusqlite/latest/rusqlite/) | SQLite et sauvegarde cohérente |
| [mdns-sd](https://docs.rs/mdns-sd/latest/mdns_sd/) | Découverte des services locaux |
| [Docker host networking](https://docs.docker.com/engine/network/drivers/host/) | Contraintes du réseau hôte sous Linux |
| [Docker Compose services](https://docs.docker.com/reference/compose-file/services/) | Options de réseau, utilisateur et volumes |
| [QNAP TS-251+](https://www.qnap.com/en/product/ts-251%2B/specs/hardware) | Cible NAS et architecture matérielle |
| [QNAP QDK](https://github.com/qnap-dev/QDK) | Distribution QPKG native |
| [Guide QDK](https://github.com/qnap-dev/QDK/blob/master/docs/QDK-Developer-Guide.md) | Cycle de vie et paramètres de packaging |
| [Licence Homebridge](https://github.com/homebridge/homebridge/blob/latest/LICENSE) | Apache-2.0 à la date de recherche |
| [Licence HAP-NodeJS](https://github.com/homebridge/HAP-NodeJS/blob/latest/LICENSE) | Apache-2.0 à la date de recherche |
| [Licence Homebridge UI](https://github.com/homebridge/homebridge-config-ui-x/blob/latest/LICENSE) | MIT à la date de recherche |

## Observations locales, sans coordonnées privées

1. Dossier de projet vide lors du démarrage ; aucun ancien code à migrer.
2. GitHub authentifié sur `magic5644` ; dépôt public demandé explicitement par le propriétaire.
3. Interface Homebridge répondante ; le propriétaire a effectué la connexion.
4. UI : Homebridge 2.4.0, UI 5.29.0, Node 24.21.0 ; Docker, Ubuntu 22.04.5, x64. Ce sont les informations du conteneur, pas une identification de QTS.
5. Plugin visible : homebridge-tahoma 2.2.64, child bridge.
6. Accessoires visibles : 11 coverings, 2 thermostats, 1 garage, 9 Switch. Cela ne prouve pas leur modèle ni la fidélité de leurs mesures.
7. Annonce mDNS développeur TaHoma présente. La connexion HTTPS avec la CA Overkiz et le nom DNS annoncés est validée ; `GET /apiVersion` répond 200 avec `protocolVersion=2026.4.5-7`.
8. Aucun token utilisé pour ce test TaHoma ; aucun setup authentifié, aucune commande physique, aucune modification de configuration, aucun redémarrage ni mise à jour.
9. Aucune mesure par processus réalisée : les gains mémoire et performances restent des hypothèses à tester.

Le propriétaire confirme : QNAP TS-251+, Celeron J1900, 2 Go, TaHoma V2, io-homecontrol et RTS. Le PIN de la passerelle, les identifiants HomeKit, les noms d'appareils et les coordonnées internes ne sont pas consignés ici.

## Inférences clairement séparées

- Une API locale joignable rend l'intégration locale plausible ; elle ne prouve pas l'accès à chaque fonction.
- Les Switch pourraient représenter des scénarios ; leur nature reste à qualifier.
- Une pile HAP Rust ancienne peut nécessiter maintenance et revue ; la date d'un commit ne prouve pas seule une vulnérabilité.
- Rust peut réduire certains coûts de runtime ; la réduction réelle dépend de l'ensemble conservé, notamment plugins, HAP, logs et fréquence des échanges.
- La mémoire totale/disponible du widget ne permet pas de déduire le coût du serveur actuel.

## Vérifications encore requises

Lecture locale authentifiée de TaHoma, modèles des équipements, version QTS/Container Station, fonctions des neuf actions, état/sécurité du garage, nature des deux thermostats, usages Matter, baseline CPU/mémoire, viabilité HAP Rust et installation native sur J1900. Aucun de ces résultats n'est présenté comme acquis.
