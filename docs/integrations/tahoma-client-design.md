# P11 — TaHoma HTTP client implementation design

Research date: October 23, 2026. Public documentation/source only; no household
files, discovery, credentials, gateway connection or equipment operation.

## Selected scope and authentication

Implement local-token mode and explicit `SomfyProvider::SomfyEurope` OAuth.
Select OAuth because current PyOverkiz routes that exact provider to its dedicated
`SomfyAuthStrategy`. Do not silently fall back to cookie login, other providers,
or local/cloud modes. Cloud authentication remains implementation evidence,
not a Somfy stability guarantee or proof of this installation's compatibility.

Until profile mappings arrive, `setup()` returns metadata-only generic devices,
with no command capabilities and honest unknown state. Preserve connector
`RawTahomaDevice` DTOs through `raw_setup()` for the P12 mapping dispatcher;
subsequently route semantic `setup()` through real mappings. Semantic command
encoding/execution belongs to P15; do not expose success-shaped stubs or invent
manufacturer command names to satisfy an anticipated interface.

## Pinned evidence

- [Official local OpenAPI, commit 3eae7a51](https://github.com/Somfy-Developer/Somfy-TaHoma-Developer-Mode/blob/3eae7a51ba55eee28649319942239aab7b139eb7/docs/openapi.yaml), API document version 1.0.
- PyOverkiz commit `7b5a0034e550e4ae27536c4d8d1a073f796d733e`:
  [provider factory](https://github.com/imicknl/python-overkiz-api/blob/7b5a0034e550e4ae27536c4d8d1a073f796d733e/pyoverkiz/auth/factory.py),
  [authentication strategies](https://github.com/imicknl/python-overkiz-api/blob/7b5a0034e550e4ae27536c4d8d1a073f796d733e/pyoverkiz/auth/strategies.py),
  [public application constants](https://github.com/imicknl/python-overkiz-api/blob/7b5a0034e550e4ae27536c4d8d1a073f796d733e/pyoverkiz/const.py),
  [token context](https://github.com/imicknl/python-overkiz-api/blob/7b5a0034e550e4ae27536c4d8d1a073f796d733e/pyoverkiz/auth/base.py),
  [response handling](https://github.com/imicknl/python-overkiz-api/blob/7b5a0034e550e4ae27536c4d8d1a073f796d733e/pyoverkiz/response_handler.py),
  [client](https://github.com/imicknl/python-overkiz-api/blob/7b5a0034e550e4ae27536c4d8d1a073f796d733e/pyoverkiz/client.py)
  and [DTOs](https://github.com/imicknl/python-overkiz-api/blob/7b5a0034e550e4ae27536c4d8d1a073f796d733e/pyoverkiz/models.py).
- Reqwest [0.13.5 builder](https://github.com/seanmonstar/reqwest/blob/v0.13.5/src/async_impl/client.rs),
  [retry policy](https://github.com/seanmonstar/reqwest/blob/v0.13.5/src/retry.rs)
  and [feature definitions](https://github.com/seanmonstar/reqwest/blob/v0.13.5/Cargo.toml).
  Registry metadata reports reqwest 0.13.5, rustls 0.23.46, rcgen 0.14.10 and
  tokio-rustls 0.26.6. Recheck/pin dependencies when implementation starts.
  Context7 resolve/query completed for reqwest and rustls; rustls `main` snippets
  describe newer APIs, so use versioned dependency documentation for actual code.

## Local routes and minimum DTOs

Base: `https://gateway-{pin}.local:{port}/enduser-mobile-web/1/enduserAPI`.
Use the selected local token as Bearer; manual IP resolution preserves URL DNS/SNI.

| Verb | Relative route | Minimum response/body |
|---|---|---|
| GET | `/setup` | Object containing `gateways` and `devices` |
| POST | `/events/register` | Object with `id` |
| POST | `/events/{listenerId}/fetch` | Event array |
| POST | `/events/{listenerId}/unregister` | Success response; local OpenAPI documents an array |
| POST | `/exec/apply` | Request `actions:[{deviceURL,commands:[{name,parameters}]}]`, optional label; response `execId`; semantic use deferred to P15 |
| GET | `/exec/current/{executionId}` | Current execution object with `id,state`; absence does not prove completion |

Encode each listener/execution identifier as one URL path segment. Never join
untrusted strings as URLs. `/apiVersion` is unauthenticated, so it cannot prove
that a supplied token is valid. Authenticate local connection using bounded setup
or another authenticated read. Listener inactivity expiry is documented as ten
minutes; fetching keeps it alive and gateway restart destroys listeners.

Raw setup DTOs need gateway `gatewayId`; device `deviceURL`, `available`,
`enabled`, `label`, `controllableName`, `states`, and `definition`. State entries
use `name`, optional `type/value`; definitions contain `commands` with
`commandName/nparams`, state names and optional UI/profile metadata. Ignore extra
fields; missing mandatory identifiers and malformed known fields are errors.

Events need `name`, optional timestamp, and variant-specific
`deviceURL/deviceStates` or `execId/newState/oldState`. The official event schema
is incomplete; current PyOverkiz models provide the latter field evidence.
Retain or safely ignore unknown event types. Execution DTOs expose at least
`id` and an honest mapped `CommandStatus`; new/unrecognized manufacturer states
must not become success. Known source states include INITIALIZED, TRANSMITTED,
IN_PROGRESS, COMPLETED, FAILED, QUEUED_GATEWAY_SIDE, QUEUED_SERVER_SIDE and
NOT_TRANSMITTED. These are manufacturer execution states, not proof of physical
position. Current-execution disappearance is not evidence of successful outcome.

## SomfyEurope account flow

Fixed account token endpoint:
`POST https://accounts.somfy.com/oauth/oauth/v2/token/jwt`.
Content type: `application/x-www-form-urlencoded`.

Password grant fields: `grant_type=password`, `client_id`, `client_secret`,
`username`, `password`. The upstream named constants `SOMFY_CLIENT_ID` and
`SOMFY_CLIENT_SECRET` are application constants published in the pinned PyOverkiz
`const.py`, documented there as embedded in mobile applications. They are not
household credentials; record their upstream provenance if used. Raw constant
values are intentionally omitted from this design note.

Successful token responses supply `access_token`, optional `refresh_token` and
`expires_in`. Store them only in private memory/session storage; never expose them
through Debug, public serialization, logs, errors or ordinary backups. The
current upstream expiry context uses a five-second skew. Reject invalid/empty
known fields rather than inventing expiration or accepting arbitrary token types.

API endpoint for this selected provider is fixed:
`https://ha101-1.overkiz.com/enduser-mobile-web/enduserAPI/`.
Send Bearer access tokens directly to that endpoint. No JSESSIONID is needed.
Refresh uses the same token endpoint with `grant_type=refresh_token`,
`client_id`, `client_secret`, `refresh_token`. Handle returned rotation/expiry;
serialize refresh through one async mutex and recheck expiration after acquiring
it. Renew before transmitting requests. No automatic replay of an uncertain
command or consuming event fetch, even after authentication failure.

PyOverkiz checks token response `message == "error.invalid.grant"` and presence
of `access_token`; 5xx token responses may be non-JSON. Map credential rejection,
401/403 and known upstream authentication error codes to a dedicated redacted
upstream-authentication error; map malformed/oversized/truncated responses to
upstream errors. Interpret status and bounded error envelopes without echoing
bodies, URLs, account names or tokens. Normal API source handling accepts 200
and 204; route-specific decoding must distinguish a valid empty response from
missing mandatory JSON. Do not trust 3xx as success.

Verify the selected `gateway_pin` against setup gateway IDs; fail when absent.
Structurally restrict device URLs to the selected gateway instead of matching
names/substrings. Other gateways in an account are not implicitly selected.
The separate PyOverkiz `SOMFY` multisite/Ginaite/BOB strategy is not this declared
provider and is outside this minimum implementation.

Unselected compatibility evidence: Homebridge
[2.2.64 package](https://github.com/dubocr/homebridge-tahoma/blob/cdfcc5988e62ec2bb45267ac0dad4b33ffabbacd/package.json)
uses overkiz-client 1.0.23,
[commit 1605feb9](https://github.com/dubocr/overkiz-client/blob/1605feb9cbe4c60682e8dba698cf093ad7ba1a7b/src/ApiClient.ts).
Its SomfyEurope flow posts `/login` with `userId,userPassword` and stores
JSESSIONID, then relogs on 401. This proves an alternative implementation exists;
it does not select a fallback or establish which flow works for this household.
Do not copy upstream generic retries or error-body logging.

## TLS provenance and client controls

Public manufacturer CA downloaded over validated HTTPS from
[Overkiz root CA](https://ca.overkiz.com/overkiz-root-ca-2048.crt), inspected with
OpenSSL in memory. RSA 2048, SHA-256 signature, CA constraints; subject/issuer
Overkiz Root CA, valid April 27, 2016 through April 27, 2036.

- SHA-256 certificate fingerprint:
  `BB:FA:96:11:1E:AE:15:E3:C6:ED:15:F3:43:27:6C:0C:27:26:03:2E:A1:9D:F5:BB:D0:67:E2:F1:CF:FC:44:DB`.
- SHA-256 downloaded PEM artifact:
  `5b4dbce7322da0718235d8d54149a3665e37822224b6a8801447f6d4d72ded15`.

Bundle/document provenance and integrity when adding the CA; no runtime CA fetch
from the gateway. Do not disable certificate or hostname verification. PyOverkiz
relaxes an OpenSSL strict-validation flag for existing gateway certificates;
that does not prove rustls compatibility and does not authorize a permissive
verifier. Actual certificate compatibility remains hardware evidence.

For reqwest 0.13.5, disable default features and select `rustls`, `json`, `form`.
Reuse clients and configure:

```rust
.retry(reqwest::retry::never())
.redirect(reqwest::redirect::Policy::none())
.no_proxy()
.timeout(Duration::from_secs(5))
```

Local trust uses `.tls_certs_only([Certificate::from_pem(ca)?])`.
Manual resolution uses `.resolve(dns_name, socket_address)` while the URL retains
the validated hostname. Cloud uses normal trusted public roots, not the gateway
CA. `Response::chunk()` permits incremental bounded reading without enabling
`stream`; enforce the 4 MiB setup limit even without Content-Length, and bound
all other responses. Do not enable automatic decompression unless decoded size
is also bounded. Never format raw reqwest/serde errors at public boundaries.

## Fictional test strategy and remaining uncertainty

Use a private constructor/test seam with fixed fictional endpoints and generated
CA/SAN certificates; keep production endpoints fixed. Bind only loopback and map
fictional DNS with `.resolve`. Test Bearer/setup, invalid token, wrong CA/name,
redirect refusal with no second host contacted, oversized/truncated/extra-field
responses, account password/refresh grants, expiry/rejection, selected gateway,
redaction and no replay after a lost response. Use subprocess environment
variables for proxy tests rather than unsafe parallel process-global mutation.

No actual authentication flow, gateway certificate, cloud session, event payload,
physical command, interoperability or OS networking behavior was tested during
this research. Fixture tests cannot close those hardware/cloud compatibility
claims. Secret files use P07 private loading; P11 must validate mode-specific
credential structure without deriving secret-revealing serialization/Debug.
