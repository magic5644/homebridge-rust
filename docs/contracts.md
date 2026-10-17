# Contrats partagés — version proposée 1

Ce document fixe les décisions dont les agents ont besoin pour travailler séparément. Le ticket P05 les transpose en types compilables. Les noms publics ci-dessous sont normatifs ; une modification impose une mise à jour du contrat, des fixtures et de tous ses consommateurs dans le même lot.

## 1. Identités et propriété

- `DeviceId`, `CommandId` et `RequestId` sont des alias de `uuid::Uuid`, sérialisés sous forme de chaîne UUID.
- `IntegrationId` est une chaîne stable configurée, par exemple `tahoma-main`.
- La clé source est `(integration_id, external_id)` ; `external_id` est l'identifiant opaque du fabricant, jamais le nom de l'appareil.
- Le premier inventaire alloue un UUID et le stocke. Renommer/réordonner les appareils ou redémarrer conserve cet UUID. Changer d'API cloud/local nécessite une correspondance explicite ; ne pas fusionner par nom.
- `Ownership` : `legacy`, `observe`, `native`. Une commande ou publication est interdite hors `native`.
- Un appareil disparu devient indisponible. Sa ligne et ses identifiants ne sont pas supprimés implicitement.

## 2. Modèle commun dans `bridge-core::model`

Notation : types Rust à créer au P05, sans imposer leur corps d'implémentation.

```rust
pub type DeviceId = uuid::Uuid;
pub type CommandId = uuid::Uuid;
pub type RequestId = uuid::Uuid;
pub type IntegrationId = String;

pub struct Device {
    pub id: DeviceId,
    pub integration_id: IntegrationId,
    pub external_id: String,
    pub name: String,
    pub capabilities: Vec<Capability>,
    pub limits: BTreeMap<StateKey, NumericLimits>,
    pub ownership: Ownership,
    pub available: bool,
    pub states: BTreeMap<StateKey, StateSample>,
}

pub enum Capability {
    OpenCloseStop, Position, Tilt, Power, Brightness,
    Temperature, Contact, Motion, Setpoint, SecurityReadOnly,
    GarageDoor, Obstruction, MomentaryGarageAction,
}
pub enum StateKey {
    PositionPct, TiltDegrees, Power, BrightnessPct,
    TemperatureCentiCelsius, ContactOpen, MotionDetected,
    TargetTemperatureCentiCelsius, SecurityState,
    DoorState, ObstructionDetected,
}
pub struct NumericLimits { pub min: i64, pub max: i64, pub step: i64 }
pub enum Scalar { Bool(bool), Integer(i64), Text(String) }
pub enum StateQuality { Confirmed, Estimated, Stale, Unknown }
pub struct StateSample {
    pub value: Option<Scalar>,
    pub quality: StateQuality,
    pub observed_at_ms: Option<u64>,
}
pub struct Snapshot { pub revision: u64, pub devices: Vec<Device> }

pub enum Command {
    Open, Close, Stop,
    SetPosition { position_pct: u8 },
    SetTilt { degrees: i16 },
    SetPower { on: bool },
    SetBrightness { percent: u8 },
    SetSetpoint { centi_celsius: i32 },
    TriggerGarage,
}
pub enum CommandOrigin { Web, HomeKit, Test }
pub struct CommandRequest {
    pub request_id: RequestId,
    pub device_id: DeviceId,
    pub command: Command,
    pub origin: CommandOrigin,
}
pub enum CommandStatus {
    Accepted, Sent, Running, Succeeded, Failed, Unknown, Cancelled,
}
pub struct CommandReceipt {
    pub command_id: CommandId,
    pub request_id: RequestId,
    pub status: CommandStatus,
}
pub struct CommandAdmission {
    pub receipt: CommandReceipt,
    pub is_new: bool,
}
pub struct CommandEnvelope {
    pub request: CommandRequest,
    pub reply: tokio::sync::oneshot::Sender<Result<CommandAdmission, BridgeError>>,
}
```

Imports partagés : `BTreeMap = std::collections::BTreeMap`, `Path/PathBuf = std::path::{Path, PathBuf}`, `Duration = std::time::Duration`, `CancellationToken = tokio_util::sync::CancellationToken` ; `BridgeError` est défini dans `bridge-core::error`. Les enums sérialisées utilisent `snake_case`. `Scalar` est taggé par `kind` et `value`, `Command` par `type`. Les variantes sans données de `Command` deviennent par exemple `{"type":"open"}`.

Règles : position 0 = complètement fermé, 100 = complètement ouvert ; inclinaison −90° à +90° si prise en charge ; température entière en centièmes de °C. `SetPosition` et `SetBrightness` acceptent 0..100 inclus ; les limites fabricant restent plus restrictives si nécessaire. Les plages/quantums sont validés avant envoi. `Unknown` impose `value = None` et aucun temps d'observation confirmé. Une estimation ne devient jamais `Confirmed` sans retour d'état physique.

Un matériel non reconnu n'obtient aucune capacité de commande. Les champs Somfy inconnus sont tolérés et conservés dans les fixtures anonymisées ; ils ne deviennent pas automatiquement des commandes génériques exposées par l'API.

Garage : `DoorState` utilise `Scalar::Text` parmi `open`, `closed`, `opening`, `closing`, `stopped` ; absence de mesure = `Unknown`. `ObstructionDetected` est un booléen confirmé uniquement si réellement mesuré. `GarageDoor` n'est activé pour HomeKit que si les caractéristiques obligatoires peuvent être renseignées honnêtement. `MomentaryGarageAction` représente une impulsion, jamais un état de porte. Une commande cyclique ne reçoit pas automatiquement `OpenCloseStop`.

`TriggerGarage` est réservé au mode impulsion explicitement qualifié au P33. Il n'est jamais utilisé pour satisfaire automatiquement un ordre HomeKit « fermer » ou « ouvrir ». P34 et P35 peuvent ajouter des modes de chauffage ou un modèle d'action groupée seulement après inventaire précis, avec mise à jour de ce contrat et de tous les consommateurs.

## 3. Erreurs

`BridgeError` expose `code() -> &'static str` et un message public sans secret. Le détail technique est dans un événement de journal expurgé. Le code d'erreur est stable entre API, CLI et tests.

| Variante / code | HTTP | Sens |
|---|---|---|
| `Validation` / `validation_error` | 422 | Valeur ou capacité invalide |
| `NotFound` / `not_found` | 404 | Appareil/commande absent |
| `NotNative` / `not_native` | 409 | Équipement en observation ou historique |
| `Conflict` / `conflict` | 409 | Même request_id avec contenu différent ou révision obsolète |
| `Busy` / `busy` | 429 | File ou opération par appareil saturée |
| `Unavailable` / `unavailable` | 503 | Box ou appareil indisponible |
| `Authentication` / `authentication_required` | 401 | Session absente/invalide |
| `Forbidden` / `forbidden` | 403 | CSRF/origine/droit invalide |
| `UpstreamAuthentication` / `upstream_authentication_failed` | 502 | Jeton fabricant rejeté ; aucun token dans la réponse |
| `Tls` / `tls_validation_failed` | 502 | CA ou nom d'hôte invalide |
| `Upstream` / `upstream_error` | 502 | Réponse fabricant inutilisable |
| `Storage` / `storage_error` | 500 | Échec transaction/stockage ; opération non annoncée réussie |

Les variantes peuvent porter un contexte privé expurgé, sans changer les codes. JSON d'erreur : `{"error":{"code":"not_native","message":"Cet appareil est en observation.","request_id":"<UUID>"}}`.

## 4. Stockage dans `bridge-core::store`

```rust
impl Store {
    pub async fn open(data_dir: &Path) -> Result<Self, BridgeError>;
    pub async fn snapshot(&self) -> Result<Snapshot, BridgeError>;
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<EventEnvelope>;
    pub async fn upsert_discovered(&self, integration_id: &str,
        devices: Vec<DiscoveredDevice>) -> Result<Snapshot, BridgeError>;
    pub async fn set_ownership(&self, id: DeviceId, ownership: Ownership,
        expected_revision: u64) -> Result<Snapshot, BridgeError>;
    pub async fn apply_updates(&self, updates: Vec<DeviceUpdate>)
        -> Result<Snapshot, BridgeError>;
    pub async fn set_integration_availability(&self, integration_id: &str,
        available: bool) -> Result<(), BridgeError>;
    pub async fn accept_command(&self, request: &CommandRequest)
        -> Result<CommandAdmission, BridgeError>;
    pub async fn mark_sent(&self, id: CommandId, execution_id: &str)
        -> Result<(), BridgeError>;
    pub async fn set_command_status(&self, id: CommandId,
        status: CommandStatus) -> Result<(), BridgeError>;
    pub async fn command(&self, id: CommandId)
        -> Result<CommandRecord, BridgeError>;
    pub async fn allocate_hap_ids(&self, device: DeviceId,
        service_keys: Vec<String>) -> Result<HapIdentity, BridgeError>;
    pub async fn backup_to(&self, destination: &Path) -> Result<(), BridgeError>;
}
```

P05 définit aussi : `DiscoveredDevice` = `external_id`, `name`, `capabilities`, `limits`, `available`, `states` ; `DeviceUpdate` = `device_id`, `available: Option<bool>`, `states` ; `CommandRecord` = requête, reçu, `created_at_ms`, `updated_at_ms`, `upstream_execution_id: Option<String>` ; `HapIdentity` = `aid: u64`, `iids: BTreeMap<String,u64>`.

P06 implémente les tables `schema_migrations`, `devices`, `hap_ids`, `commands`, `sessions`, `settings`. Contraintes uniques sur la clé source et sur request_id ; transactions pour identité/révision/commande. Les mappings service→IID sont conservés après retrait d'une capacité. Un second processus ouvrant le même répertoire en écriture est refusé avec un verrou de processus.

La révision est un entier persistant croissant pour toute modification de l'instantané. Les commandes ont leur propre état et ne simulent pas de modification de la mesure physique. Les événements d'état sont émis seulement après validation de la transaction.

Un appareil nouvellement découvert est créé en `observe`. Un nouvel inventaire ne modifie jamais la propriété déjà choisie. La disponibilité d'intégration est persistée dans `settings` et émet `IntegrationChanged` seulement après commit.

Idempotence : même request_id et même commande normalisée → reçu existant, sans nouvel envoi ; même identifiant et contenu différent → conflit. Conservation au moins 24 h, maximum 10 000 commandes ; à saturation dans la fenêtre, refuser les nouvelles commandes plutôt que supprimer une clé encore valide. Les commandes non terminales au redémarrage passent à `Unknown`, sans réémission.

`CommandAdmission.is_new` est décidé transactionnellement. Seul `true` autorise le routeur à transmettre ; une duplication avec reçu encore `Accepted` ne doit pas produire un second envoi. Le champ `is_new` reste interne : l'API renvoie le reçu avec HTTP 202 ou 200 selon le cas. `mark_sent` persiste ensemble l'identifiant d'exécution fabricant et le statut `Sent`.

## 5. Événements

```rust
pub struct EventEnvelope { pub sequence: u64, pub event: BridgeEvent }
pub enum BridgeEvent {
    SnapshotChanged { revision: u64, device_ids: Vec<DeviceId> },
    CommandChanged { command_id: CommandId, status: CommandStatus },
    IntegrationChanged { integration_id: IntegrationId, available: bool },
    ResyncRequired,
}
```

`sequence` est propre au démarrage courant, sans promesse de reprise durable. L'événement Web ne contient pas de token, d'URL fabricant ou de clé d'appairage. Le client récupère toujours un instantané après reconnexion. Une perte dans le broadcast provoque `ResyncRequired`, puis un nouvel instantané. Les consommateurs HomeKit font la même réconciliation au lieu d'ignorer un retard.

## 6. Interfaces des adaptateurs

```rust
// integration-tahoma
pub async fn discover(timeout: Duration)
    -> Result<Vec<DiscoveredGateway>, BridgeError>;
impl TahomaClient {
    pub fn new(config: TahomaConfig, token: String) -> Result<Self, BridgeError>;
    pub async fn setup(&self) -> Result<Vec<DiscoveredDevice>, BridgeError>;
    pub async fn register_listener(&self) -> Result<String, BridgeError>;
    pub async fn fetch_events(&self, listener: &str)
        -> Result<Vec<TahomaEvent>, BridgeError>;
    pub async fn unregister_listener(&self, listener: &str)
        -> Result<(), BridgeError>;
    pub async fn execute(&self, external_id: &str, command: &Command)
        -> Result<String, BridgeError>; // identifiant d'exécution fabricant
    pub async fn execution(&self, execution_id: &str)
        -> Result<TahomaExecution, BridgeError>;
}
pub fn map_device(raw: &RawTahomaDevice) -> Result<DiscoveredDevice, BridgeError>;
pub fn encode_command(raw: &RawTahomaDevice, command: &Command)
    -> Result<serde_json::Value, BridgeError>;

// transport-homekit : même signature pour les deux backends
pub async fn run(config: HomeKitConfig, store: Store,
    commands: tokio::sync::mpsc::Sender<CommandEnvelope>,
    cancel: tokio_util::sync::CancellationToken) -> Result<(), BridgeError>;
```

`RawTahomaDevice`, `TahomaEvent`, `TahomaExecution` et DTO d'exécution restent privés au connecteur, à l'exception des types explicitement utilisés par ses fonctions publiques. P11 définit leurs champs depuis les fixtures et l'OpenAPI ; ne pas générer l'API Somfy entière. `TahomaExecution` expose au minimum `id: String` et `status: CommandStatus`.

`DiscoveredGateway` et `TahomaConfig` sont définis au P05 : nom DNS TLS, port, PIN attendu privé, adresse IP résolue optionnelle, `integration_id` ; `TahomaConfig` ajoute `token_file: PathBuf` et `ca_file: PathBuf`. `HomeKitConfig` : nom, port TCP, interface réseau optionnelle, `state_dir: PathBuf`, backend sélectionné au P04. Le PIN d'appairage est généré et stocké dans l'état privé, jamais dans un fichier d'exemple.

Le transport HomeKit utilise le Store pour allouer ses AID/IID, souscrire aux événements et relire un instantané après perte d'événements. Il ne conserve pas seulement une copie initiale dont il serait impossible de se resynchroniser.

P14 fournit `run_events(client: TahomaClient, store: Store, cancel: CancellationToken) -> Result<(), BridgeError>` async ; les mises à jour transitent par le Store, qui publie les événements après commit. `Store` est clonable et partage le même worker et le même broadcast de capacité 256. Chaque consommateur appelle `subscribe` puis lit un instantané, et se réconcilie sur retard.

P18 fournit `CommandRouter::submit(&self, CommandRequest) -> Result<CommandAdmission, BridgeError>` async, partagé par Web et HomeKit. Il utilise le Store et le connecteur concret ; aucune interface de plugins dynamiques n'est à inventer.

## 7. API Web version 1

Préfixe `/api/v1`. Le schéma OpenAPI produit au P08 est la transposition de ces contrats, pas une seconde spécification indépendante. Authentification exigée sauf routes de santé et parcours de création du premier administrateur.

| Méthode / chemin | Entrée / sortie et règle |
|---|---|
| `GET /health/live` | 200 si processus répond ; aucune donnée privée |
| `GET /health/ready` | 200 si stockage/API prêts, 503 sinon ; panne box = dégradation dans l'API authentifiée |
| `GET /api/v1/setup/status` | Uniquement `initialized: bool` |
| `POST /api/v1/setup/admin` | Code éphémère + identifiant + mot de passe ; 201 une seule fois |
| `POST /api/v1/auth/login` | Crée une session ; 204 + cookie ; erreur générique |
| `POST /api/v1/auth/logout` | Détruit la session ; 204 |
| `GET /api/v1/auth/session` | Utilisateur courant et jeton CSRF de session |
| `GET /api/v1/system` | Version, uptime, RSS si disponible, état des intégrations et backend HAP |
| `GET /api/v1/devices` | Instantané public : révision + appareils ; external_id omis |
| `POST /api/v1/devices/{id}/commands` | `{request_id, command}` ; 202 nouveau reçu, 200 reçu idempotent |
| `GET /api/v1/commands/{id}` | État, dates et erreur publique de la commande |
| `PUT /api/v1/devices/{id}/ownership` | `{ownership, expected_revision}` ; observation par défaut ; pas de désactivation distante de Homebridge |
| `GET /api/v1/events` | SSE de session ; heartbeat 15 s ; resynchronisation à chaque reconnexion |
| `GET /api/v1/integrations/tahoma` | Configuration sans jeton, état de connexion et dernière erreur expurgée |
| `PUT /api/v1/integrations/tahoma` | Paramètres et token write-only ; validation, pas d'écho du secret |
| `POST /api/v1/integrations/tahoma/test` | Test TLS/auth et lecture ; aucune commande |
| `GET /api/v1/homekit` | Backend, statut et appareils publiés ; sans PIN |
| `GET /api/v1/homekit/pairing` | QR/PIN pour session administrateur ; `Cache-Control: no-store` |
| `GET /api/v1/logs?limit=...` | Maximum 500 lignes expurgées |
| `POST /api/v1/backups` | Sauvegarde expurgée téléchargeable ; secrets explicitement exclus |
| `POST /api/v1/restore/validate` | Valide une archive expurgée et renvoie un rapport ; n'applique rien |

La restauration qui applique des données est une commande CLI locale au premier lot. Pas de route redémarrant le NAS, modifiant Docker ou exécutant un shell. Les mises à jour sont assurées par l'installation Docker/native, avec sauvegarde préalable.

JSON d'une commande :

```json
{
  "request_id": "00000000-0000-4000-8000-000000000010",
  "command": { "type": "set_position", "position_pct": 80 }
}
```

L'API n'accepte jamais directement un nom de commande Somfy ou une URL arbitraire dans cette route. Le connecteur sélectionne les opérations autorisées par les capacités inventoriées.

## 8. Limites initiales et arrêt

| Ressource | Limite proposée |
|---|---|
| JSON API ordinaire | 256 Kio ; 413 au-delà |
| Réponse TaHoma setup | 4 Mio décompressés ; erreur explicite au-delà |
| File de commandes admises | 32 globales, une commande ordinaire active par appareil ; stop prioritaire admis pendant un mouvement |
| Attente avant transmission | 5 s ; expiration avant envoi si dépassée |
| Requête HTTP fabricant | Timeout 5 s ; une écriture en vol par box |
| Suivi de déplacement | Timeout 120 s ; état `Unknown` si résultat non prouvé |
| Broadcast d'événements | 256 événements ; resync pour lecteur lent |
| Clients SSE | 16 simultanés ; 429 au-delà |
| Requêtes au worker SQLite | 64 en attente ; refus borné |
| Historique de logs en mémoire | 500 lignes, 2 Kio maximum par ligne |
| Archive de restauration | 50 Mio compressés, 200 Mio extraits, 1 000 entrées |
| Arrêt SIGTERM | Arrêt admission, annulation flux, clôture stockage, sortie ≤ 10 s |

Les seuils se modifient seulement avec mesure et test associé. Une file pleine n'entraîne jamais un `spawn` sans limite. La commande `Stop` peut remplacer une position non encore envoyée ; elle ne réexécute pas une opération physique déjà incertaine.

Une commande déjà envoyée n'est déclarée `Cancelled` que sur preuve fabricant ; sinon elle reste en suivi ou devient `Unknown`. La fin d'un déplacement n'interdit pas la réception de sa commande Stop. Le routeur réserve une place de traitement prioritaire à Stop en remplaçant si nécessaire une position non transmise ; il n'augmente pas la borne totale de 32.

La propriété et la disponibilité sont revérifiées juste avant transmission. Passer à `observe` annule les commandes non envoyées et empêche leur départ ultérieur ; une commande déjà transmise conserve son suivi ou devient incertaine, sans promesse de l'arrêter physiquement.

Les handlers déjà ouverts et les SSE doivent être annulables pour que l'arrêt gracieux ne reste pas suspendu. L'horloge monotone mesure les délais ; les temps Unix servent à la persistance et à l'affichage.

## 9. Fixtures communes

P03 crée uniquement des exemples inventés sous `fixtures/tahoma/` : `setup_io.json`, `setup_rts.json`, `setup_switch.json`, `setup_sensor.json`, `setup_unknown.json`, `events_state.json`, `events_execution.json`, `events_listener_expired.json`, `setup_extra_fields.json` et `setup_invalid.json`.

Les exemples utilisent un PIN fictif et des URLs cohérentes mais non réelles. Une conversion de captures réelles doit conserver commandes, types et bornes utiles, tout en remplaçant identifiants, noms de pièces, localisation, comptes et clés. Les formes d'événements réels anonymisés complètent l'OpenAPI si celle-ci manque de détails ; ne pas inventer leur structure en la présentant comme confirmée.

P09 ajoute `fixtures/api/` : instantané, commande, erreurs et resync. P20 ajoute `fixtures/homekit/` : projection et états indisponibles. Les trois couches utilisent la même convention fermé=0/ouvert=100 et les mêmes UUID fictifs.
