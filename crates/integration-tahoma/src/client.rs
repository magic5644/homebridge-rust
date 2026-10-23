//! No redirects, proxy, automatic retries, or cross-mode authentication fallback.
use crate::dto::*;
use bridge_core::{
    config::{SomfyProvider, TahomaConfig},
    error::BridgeError,
    model::{CommandStatus, DiscoveredDevice},
};
use reqwest::{
    Client, Method, Url,
    header::{AUTHORIZATION, HeaderValue},
};
use serde::de::DeserializeOwned;
use std::{io::Read, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::Instant};

const MAX_RESPONSE: usize = 4 * 1024 * 1024;
const ACCOUNT_TOKEN_URL: &str = "https://accounts.somfy.com/oauth/oauth/v2/token/jwt";
const ACCOUNT_API_URL: &str = "https://ha101-1.overkiz.com/enduser-mobile-web/enduserAPI/";
// Published mobile application constants, not household secrets; see certs/PROVENANCE.md.
const SOMFY_CLIENT_ID: &str =
    "0d8e920c-1478-11e7-a377-02dd59bd3041_1ewvaqmclfogo4kcsoo0c8k4kso884owg08sg8c40sk4go4ksg";
const SOMFY_CLIENT_SECRET: &str = "12k73w1n540g8o4cokg0cw84cog840k84cwggscwg884004kgk";

/// Loaded private credentials. Deliberately neither Debug nor serializable.
pub enum TahomaCredentials {
    LocalToken { token: String },
    SomfyAccount { username: String, password: String },
}
#[derive(Clone)]
pub struct TahomaClient {
    http: Client,
    base: Url,
    gateway_pin: String,
    auth: Arc<Auth>,
}
enum Auth {
    Local(HeaderValue),
    Account {
        token_url: Url,
        session: Mutex<Session>,
    },
}
struct Session {
    access: HeaderValue,
    refresh: Option<String>,
    expires_at: Instant,
}
#[derive(serde::Deserialize)]
struct Token {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: u64,
    token_type: Option<String>,
}
fn builder() -> reqwest::ClientBuilder {
    Client::builder()
        .tls_backend_rustls()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .timeout(Duration::from_secs(5))
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .no_zstd()
        .tls_sslkeylogfile(false)
        .pool_max_idle_per_host(1)
}
fn bearer(token: &str) -> Result<HeaderValue, BridgeError> {
    if token.is_empty() || token.len() > 65536 || !token.bytes().all(|b| b.is_ascii_graphic()) {
        return Err(BridgeError::Validation);
    }
    let mut header =
        HeaderValue::from_str(&format!("Bearer {token}")).map_err(|_| BridgeError::Validation)?;
    header.set_sensitive(true);
    Ok(header)
}
fn valid_pin(pin: &str) -> bool {
    pin.len() == 14
        && pin.bytes().enumerate().all(|(i, b)| {
            if i == 4 || i == 9 {
                b == b'-'
            } else {
                b.is_ascii_digit()
            }
        })
}
fn transport_error(error: reqwest::Error) -> BridgeError {
    // reqwest errors include private URLs; inspect types without formatting them.
    let mut cause: Option<&(dyn std::error::Error + 'static)> = Some(&error);
    while let Some(source) = cause {
        if source.is::<rustls::Error>() {
            return BridgeError::Tls;
        }
        cause = match source.downcast_ref::<std::io::Error>() {
            Some(io) => io
                .get_ref()
                .map(|e| e as &(dyn std::error::Error + 'static)),
            None => source.source(),
        };
    }
    if error.is_connect() {
        BridgeError::Unavailable
    } else {
        BridgeError::Upstream
    }
}

async fn read(mut response: reqwest::Response) -> Result<Vec<u8>, BridgeError> {
    let status = response.status();
    if status == 401 || status == 403 {
        return Err(BridgeError::UpstreamAuthentication);
    }
    if response
        .content_length()
        .is_some_and(|n| n > MAX_RESPONSE as u64)
    {
        return Err(BridgeError::Upstream);
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(transport_error)? {
        if chunk.len() > MAX_RESPONSE - bytes.len() {
            return Err(BridgeError::Upstream);
        }
        bytes.extend_from_slice(&chunk);
    }
    if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) {
        if value.get("message").and_then(|v| v.as_str()) == Some("error.invalid.grant")
            || matches!(
                value.get("errorCode").and_then(|v| v.as_str()),
                Some("AUTHENTICATION_ERROR" | "RESOURCE_ACCESS_DENIED")
            )
        {
            return Err(BridgeError::UpstreamAuthentication);
        }
        if value.get("errorCode").is_some() {
            return Err(BridgeError::Upstream);
        }
    }
    if status != 200 && status != 204 {
        return Err(BridgeError::Upstream);
    }
    Ok(bytes)
}
fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, BridgeError> {
    serde_json::from_slice(bytes).map_err(|_| BridgeError::Upstream)
}
async fn token(
    http: &Client,
    url: Url,
    grant: &[(&str, &str)],
    old_refresh: Option<String>,
) -> Result<Session, BridgeError> {
    let mut form = vec![
        ("client_id", SOMFY_CLIENT_ID),
        ("client_secret", SOMFY_CLIENT_SECRET),
    ];
    form.extend_from_slice(grant);
    let response = http
        .post(url)
        .form(&form)
        .send()
        .await
        .map_err(transport_error)?;
    let token: Token = decode(&read(response).await?)?;
    if token.expires_in == 0
        || token
            .token_type
            .as_ref()
            .is_some_and(|v| !v.eq_ignore_ascii_case("Bearer"))
        || token.refresh_token.as_ref().is_some_and(|v| {
            v.is_empty() || v.len() > 65536 || !v.bytes().all(|b| b.is_ascii_graphic())
        })
    {
        return Err(BridgeError::Upstream);
    }
    let expires_at = Instant::now()
        .checked_add(Duration::from_secs(token.expires_in.saturating_sub(5)))
        .ok_or(BridgeError::Upstream)?;
    Ok(Session {
        access: bearer(&token.access_token).map_err(|_| BridgeError::Upstream)?,
        refresh: token.refresh_token.or(old_refresh),
        expires_at,
    })
}
impl TahomaClient {
    /// Authenticates with a bounded setup read; no mDNS or equipment command is run.
    pub async fn connect(
        config: TahomaConfig,
        credentials: TahomaCredentials,
    ) -> Result<Self, BridgeError> {
        config.validate()?;
        match (config, credentials) {
            (
                TahomaConfig::LocalToken {
                    tls_dns_name,
                    port,
                    expected_pin,
                    resolved_ip,
                    ca_file,
                    ..
                },
                TahomaCredentials::LocalToken { token },
            ) => {
                if !valid_pin(&expected_pin)
                    || tls_dns_name != format!("gateway-{expected_pin}.local")
                {
                    return Err(BridgeError::Validation);
                }
                let mut ca = Vec::new();
                std::fs::File::open(ca_file)
                    .map_err(|_| BridgeError::Tls)?
                    .take(65537)
                    .read_to_end(&mut ca)
                    .map_err(|_| BridgeError::Tls)?;
                if ca.len() > 65536 {
                    return Err(BridgeError::Tls);
                }
                let certificate =
                    reqwest::Certificate::from_pem(&ca).map_err(|_| BridgeError::Tls)?;
                let mut http = builder().tls_certs_only([certificate]);
                if let Some(ip) = resolved_ip {
                    http = http.resolve(&tls_dns_name, (ip, port).into())
                }
                let client = Self {
                    http: http.build().map_err(|_| BridgeError::Tls)?,
                    base: Url::parse(&format!(
                        "https://{tls_dns_name}:{port}/enduser-mobile-web/1/enduserAPI/"
                    ))
                    .map_err(|_| BridgeError::Validation)?,
                    gateway_pin: expected_pin,
                    auth: Arc::new(Auth::Local(bearer(&token)?)),
                };
                client.raw_setup().await?;
                Ok(client)
            }
            (
                TahomaConfig::SomfyAccount {
                    provider: SomfyProvider::SomfyEurope,
                    gateway_pin,
                    ..
                },
                TahomaCredentials::SomfyAccount { username, password },
            ) => {
                if !valid_pin(&gateway_pin)
                    || username.trim().is_empty()
                    || password.is_empty()
                    || username.len() > 65536
                    || password.len() > 65536
                {
                    return Err(BridgeError::Validation);
                }
                Self::account(
                    builder().build().map_err(|_| BridgeError::Tls)?,
                    Url::parse(ACCOUNT_API_URL).map_err(|_| BridgeError::Validation)?,
                    Url::parse(ACCOUNT_TOKEN_URL).map_err(|_| BridgeError::Validation)?,
                    gateway_pin,
                    username,
                    password,
                )
                .await
            }
            _ => Err(BridgeError::Validation),
        }
    }
    async fn account(
        http: Client,
        base: Url,
        token_url: Url,
        gateway_pin: String,
        username: String,
        password: String,
    ) -> Result<Self, BridgeError> {
        let session = token(
            &http,
            token_url.clone(),
            &[
                ("grant_type", "password"),
                ("username", &username),
                ("password", &password),
            ],
            None,
        )
        .await?;
        let client = Self {
            http,
            base,
            gateway_pin,
            auth: Arc::new(Auth::Account {
                token_url,
                session: Mutex::new(session),
            }),
        };
        client.raw_setup().await?;
        Ok(client)
    }
    async fn authorization(&self) -> Result<HeaderValue, BridgeError> {
        match &*self.auth {
            Auth::Local(header) => Ok(header.clone()),
            Auth::Account { token_url, session } => {
                let mut session = session.lock().await;
                if Instant::now() >= session.expires_at {
                    let refresh = session
                        .refresh
                        .clone()
                        .ok_or(BridgeError::UpstreamAuthentication)?;
                    *session = token(
                        &self.http,
                        token_url.clone(),
                        &[("grant_type", "refresh_token"), ("refresh_token", &refresh)],
                        Some(refresh.clone()),
                    )
                    .await?;
                    if Instant::now() >= session.expires_at {
                        return Err(BridgeError::UpstreamAuthentication);
                    }
                }
                Ok(session.access.clone())
            }
        }
    }
    fn url(&self, segments: &[&str]) -> Result<Url, BridgeError> {
        if segments
            .iter()
            .any(|s| s.is_empty() || *s == "." || *s == ".." || s.len() > 4096)
        {
            return Err(BridgeError::Validation);
        }
        let mut url = self.base.clone();
        url.path_segments_mut()
            .map_err(|_| BridgeError::Validation)?
            .pop_if_empty()
            .extend(segments);
        Ok(url)
    }
    async fn request(
        &self,
        method: Method,
        segments: &[&str],
        body: Option<&TahomaActionGroup>,
    ) -> Result<Vec<u8>, BridgeError> {
        let url = self.url(segments)?;
        let mut request = self
            .http
            .request(method, url)
            .header(AUTHORIZATION, self.authorization().await?);
        if let Some(body) = body {
            request = request.json(body)
        }
        read(request.send().await.map_err(transport_error)?).await
    }
    fn selected_device(&self, url: &str) -> bool {
        let Some((scheme, rest)) = url.split_once("://") else {
            return false;
        };
        let Some((gateway, path)) = rest.split_once('/') else {
            return false;
        };
        !scheme.is_empty()
            && scheme
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'-' | b'.'))
            && gateway == self.gateway_pin
            && !path.is_empty()
            && !url.chars().any(char::is_whitespace)
    }
    pub async fn raw_setup(&self) -> Result<Vec<RawTahomaDevice>, BridgeError> {
        let setup: Setup = decode(&self.request(Method::GET, &["setup"], None).await?)?;
        if !setup
            .gateways
            .iter()
            .any(|g| g.gateway_id == self.gateway_pin)
        {
            return Err(BridgeError::Validation);
        }
        if setup
            .devices
            .iter()
            .any(|d| d.device_url.trim().is_empty() || d.controllable_name.trim().is_empty())
        {
            return Err(BridgeError::Upstream);
        }
        Ok(setup
            .devices
            .into_iter()
            .filter(|d| self.selected_device(&d.device_url))
            .collect())
    }
    /// Metadata only until the profile dispatcher supplies real capabilities and states.
    pub async fn setup(&self) -> Result<Vec<DiscoveredDevice>, BridgeError> {
        Ok(self
            .raw_setup()
            .await?
            .into_iter()
            .map(|raw| DiscoveredDevice {
                external_id: raw.device_url,
                name: raw.label,
                available: raw.available && raw.enabled,
                capabilities: Vec::new(),
                limits: Default::default(),
                states: Default::default(),
            })
            .collect())
    }
    pub async fn register_listener(&self) -> Result<String, BridgeError> {
        let listener: Listener = decode(
            &self
                .request(Method::POST, &["events", "register"], None)
                .await?,
        )?;
        self.url(&[&listener.id])
            .map_err(|_| BridgeError::Upstream)?;
        Ok(listener.id)
    }
    pub async fn fetch_events(&self, listener: &str) -> Result<Vec<TahomaEvent>, BridgeError> {
        let events: Vec<TahomaEvent> = decode(
            &self
                .request(Method::POST, &["events", listener, "fetch"], None)
                .await?,
        )?;
        if events.iter().any(|e| {
            let missing = |v: &Option<String>| v.as_ref().is_none_or(|s| s.trim().is_empty());
            e.name.trim().is_empty()
                || e.device_states
                    .as_ref()
                    .is_some_and(|states| states.iter().any(|s| s.name.trim().is_empty()))
                || matches!(
                    e.name.as_str(),
                    "DeviceStateChangedEvent"
                        | "DeviceAvailableEvent"
                        | "DeviceUnavailableEvent"
                        | "DeviceCreatedEvent"
                        | "DeviceRemovedEvent"
                        | "DeviceUpdatedEvent"
                ) && missing(&e.device_url)
                || e.name == "DeviceStateChangedEvent" && e.device_states.is_none()
                || e.name == "ExecutionStateChangedEvent"
                    && (missing(&e.exec_id) || missing(&e.new_state) || missing(&e.old_state))
        }) {
            return Err(BridgeError::Upstream);
        }
        Ok(events
            .into_iter()
            .filter(|e| {
                e.device_url
                    .as_deref()
                    .is_none_or(|url| self.selected_device(url))
            })
            .collect())
    }
    pub async fn unregister_listener(&self, listener: &str) -> Result<(), BridgeError> {
        let bytes = self
            .request(Method::POST, &["events", listener, "unregister"], None)
            .await?;
        if !bytes.is_empty() {
            let _: Vec<serde_json::Value> = decode(&bytes)?;
        }
        Ok(())
    }
    /// P15 must validate native ownership and map commands before calling this raw route.
    pub async fn apply_raw(&self, group: &TahomaActionGroup) -> Result<String, BridgeError> {
        if group.actions.is_empty()
            || group.actions.iter().any(|a| {
                !self.selected_device(&a.device_url)
                    || a.commands.is_empty()
                    || a.commands.iter().any(|c| c.name.trim().is_empty())
            })
        {
            return Err(BridgeError::Validation);
        }
        let execution: ExecutionId = decode(
            &self
                .request(Method::POST, &["exec", "apply"], Some(group))
                .await?,
        )?;
        self.url(&[&execution.exec_id])
            .map_err(|_| BridgeError::Upstream)?;
        Ok(execution.exec_id)
    }
    pub async fn execution(&self, id: &str) -> Result<TahomaExecution, BridgeError> {
        let execution: Execution = decode(
            &self
                .request(Method::GET, &["exec", "current", id], None)
                .await?,
        )?;
        if execution.id != id || execution.state.is_empty() {
            return Err(BridgeError::Upstream);
        }
        let status = match execution.state.as_str() {
            "INITIALIZED" | "QUEUED_GATEWAY_SIDE" | "QUEUED_SERVER_SIDE" => CommandStatus::Accepted,
            "TRANSMITTED" => CommandStatus::Sent,
            "IN_PROGRESS" => CommandStatus::Running,
            "COMPLETED" => CommandStatus::Succeeded,
            "FAILED" | "NOT_TRANSMITTED" => CommandStatus::Failed,
            _ => CommandStatus::Unknown,
        };
        Ok(TahomaExecution {
            id: execution.id,
            status,
        })
    }
}
#[cfg(test)]
#[path = "../tests/support/account.rs"]
mod account_contract;
