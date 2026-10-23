CREATE TABLE schema_migrations (version INTEGER PRIMARY KEY);
CREATE TABLE devices (
    id TEXT PRIMARY KEY,
    integration_id TEXT NOT NULL,
    external_id TEXT NOT NULL,
    body TEXT NOT NULL,
    UNIQUE (integration_id, external_id)
);
CREATE TABLE hap_ids (
    device_id TEXT PRIMARY KEY REFERENCES devices(id),
    aid INTEGER NOT NULL UNIQUE CHECK (aid > 1),
    body TEXT NOT NULL
);
CREATE TABLE commands (
    id TEXT PRIMARY KEY,
    request_id TEXT NOT NULL UNIQUE,
    created_at_ms INTEGER NOT NULL,
    body TEXT NOT NULL
);
CREATE TABLE sessions (id TEXT PRIMARY KEY, body TEXT NOT NULL);
CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
INSERT INTO settings VALUES ('revision', '0');
INSERT INTO schema_migrations VALUES (1);
PRAGMA user_version = 1;
