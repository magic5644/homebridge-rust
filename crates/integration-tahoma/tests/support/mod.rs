#![allow(dead_code)]
use bridge_core::config::TahomaConfig;
use std::sync::{Arc, Mutex};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task::JoinHandle,
};
use tokio_rustls::{
    TlsAcceptor,
    rustls::{
        ServerConfig,
        pki_types::{CertificateDer, PrivatePkcs8KeyDer},
    },
};

pub const PIN: &str = "0000-1111-2222";
pub const DNS: &str = "gateway-0000-1111-2222.local";
pub const SETUP: &str = r#"{"gateways":[{"gatewayId":"0000-1111-2222"}],"devices":[{"deviceURL":"io://0000-1111-2222/1234","available":true,"enabled":true,"label":"Fixture blind","controllableName":"io:Fixture","states":[],"definition":{"commands":[],"states":[]},"future":42}],"extra":true}"#;
pub fn response(status: u16, body: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 {status} Fictional\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .into_bytes()
}
pub struct Server {
    pub port: u16,
    pub requests: Arc<Mutex<Vec<String>>>,
    task: JoinHandle<()>,
}
impl Drop for Server {
    fn drop(&mut self) {
        self.task.abort();
    }
}
impl Server {
    pub async fn start(replies: Vec<Vec<u8>>) -> Self {
        Self::delayed(replies, std::time::Duration::ZERO).await
    }
    pub async fn delayed(replies: Vec<Vec<u8>>, delay: std::time::Duration) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                vec![CertificateDer::from(
                    include_bytes!("../fixtures/tls/server.der").to_vec(),
                )],
                PrivatePkcs8KeyDer::from(include_bytes!("../fixtures/tls/server-key.der").to_vec())
                    .into(),
            )
            .unwrap();
        let acceptor = TlsAcceptor::from(Arc::new(config));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let seen = requests.clone();
        let task = tokio::spawn(async move {
            let mut replies = replies.into_iter();
            loop {
                let (socket, _) = listener.accept().await.unwrap();
                let Ok(mut stream) = acceptor.accept(socket).await else {
                    continue;
                };
                let mut buf = Vec::new();
                let mut temp = [0; 4096];
                while let Ok(n) = stream.read(&mut temp).await {
                    if n == 0 {
                        break;
                    }
                    buf.extend_from_slice(&temp[..n]);
                    if let Some(i) = buf.windows(4).position(|v| v == b"\r\n\r\n") {
                        let h = String::from_utf8_lossy(&buf[..i]);
                        let len = h
                            .lines()
                            .find_map(|l| {
                                l.to_ascii_lowercase()
                                    .strip_prefix("content-length: ")
                                    .and_then(|v| v.parse::<usize>().ok())
                            })
                            .unwrap_or(0);
                        if buf.len() >= i + 4 + len {
                            break;
                        }
                    }
                }
                seen.lock()
                    .unwrap()
                    .push(String::from_utf8_lossy(&buf).into_owned());
                let reply = replies
                    .next()
                    .unwrap_or_else(|| response(500, "unexpected fixture request"));
                tokio::time::sleep(delay).await;
                let _ = stream.write_all(&reply).await;
                let _ = stream.shutdown().await;
            }
        });
        Self {
            port,
            requests,
            task,
        }
    }
    pub fn config(&self) -> TahomaConfig {
        TahomaConfig::LocalToken {
            integration_id: "fixture".into(),
            tls_dns_name: DNS.into(),
            port: self.port,
            expected_pin: PIN.into(),
            resolved_ip: Some("127.0.0.1".parse().unwrap()),
            token_file: "unused-fixture-token".into(),
            ca_file: std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/tls/ca.pem"),
        }
    }
    pub fn count(&self) -> usize {
        self.requests.lock().unwrap().len()
    }
}
