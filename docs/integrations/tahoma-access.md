# TaHoma access: local token or Somfy account

Owner decision, October 17, 2026: the Rust connector must offer both modes. This document specifies future P05/P07/P11/P23 work; neither mode is implemented by P03.

| Mode | Recommendation and limits |
|---|---|
| `local_token` | Recommended default. Direct LAN connection using a gateway developer token; avoids storing the Somfy account password and cloud dependency for locally supported operations. Validate gateway CA and DNS. |
| `somfy_account` | Explicit alternative using the Somfy account and selected supported region/gateway. Requires Internet/manufacturer services; authentication and available features may change. Validate the current flow before implementation. |

A local token is a powerful secret: it authorizes control as well as reads and is not a read-only credential. The read-only boundary comes from application ownership/mode and the operations allowed in the session. Protect either credential against disclosure. Local mode reduces account-password exposure; it is not a guarantee of lower risk in every network configuration.

No silent cloud fallback, automatic mode switching or replay of an uncertain command. Changing modes requires explicit device mapping; matching names does not establish identity. If local mode lacks a used stored scenario, retain its existing Homebridge path until the selected replacement is validated. Do not claim local/cloud feature parity.

## Obtain a local token in the Somfy application

Follow the [official Somfy instructions](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode#getting-started), checked October 17, 2026. Menu translations and placement may vary:

1. Open **TaHoma by Somfy**, then **Configure the installation** and the TaHoma box parameters.
2. If Developer Mode is unavailable, tap the gateway PIN seven times to expose/activate it. The observed installation already advertises the developer API; do not reactivate or reset it unnecessarily.
3. In the Developer Mode menu, generate a token yourself. Accept any required terms yourself.
4. Copy it directly into private local storage: Somfy displays it only when created. Do not send it through chat, commit it, include it in a URL or put it in a shell argument.

The future setup UI accepts a write-only token, with a link to this guide. P07 reads a private secret file (0600 inside a 0700 directory); public config stores its path only. The agent does not need to see the token to review an owner-collected inventory. Token creation/revocation is an operator action, separate from read-only inventory.

## Connection and account handling

Local TLS must validate the DNS identity and manufacturer CA, available at [Overkiz root CA](https://ca.overkiz.com/overkiz-root-ca-2048.crt). A manual IP override preserves the validated DNS name; never disable certificate checks. Do not follow token-bearing redirects or use ambient gateway proxies. Pin/document the CA provenance and fingerprint at P11.

Account mode uses private write-only credentials, restricted local secret/session storage and validated manufacturer HTTPS endpoints. Never expose account identifiers, passwords, session cookies or refresh tokens through configuration GETs, logs, backups, fixtures or CI. Authenticate only against the selected supported provider; no arbitrary credential destination. P11 must verify current login/session renewal, account/gateway selection, rejection/expiry handling and bounds using fictional HTTP simulations, without implementing a JavaScript server or adding a Python runtime.

[PyOverkiz authentication documentation](https://github.com/imicknl/python-overkiz-api/blob/main/docs/getting-started.md), consulted through Context7, is secondary implementation evidence for token/account modes, not a Somfy cloud API stability guarantee. Its suggestion to disable TLS for IP connections is expressly excluded here. The [official local OpenAPI](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode/blob/main/docs/openapi.yaml) does not prove stored-scenario parity. Current Homebridge inventory uses Somfy Europe/cloud; importing those projections does not demonstrate local support.

The test-connection action performs authentication and bounded inventory reads only. `legacy` and `observe` prohibit device/scenario commands and HomeKit publication in either mode.
