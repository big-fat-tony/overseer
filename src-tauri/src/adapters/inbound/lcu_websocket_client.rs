use crate::adapters::inbound::lcu_websocket_adapter::LcuWebSocketAdapter;
use crate::adapters::inbound::no_cert_verification::verifier;
use crate::domain::lcu_socket_frame::LcuSocketFrame;
use crate::domain::ports::LockfilePort;

use base64::Engine;
use futures_util::{SinkExt, StreamExt};
use futures_util::future::Either;
use tokio::sync::oneshot;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{tungstenite, Connector};

use futures_util::future::select;
use std::sync::{Arc, Mutex};

pub struct LcuWebSocketClient {
    adapter: LcuWebSocketAdapter,
    lockfile: Box<dyn LockfilePort>,
    name: String,
    close_signal: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl LcuWebSocketClient {
    pub fn new(
        adapter: LcuWebSocketAdapter,
        lockfile: Box<dyn LockfilePort>,
        name: String,
    ) -> Self {
        Self {
            adapter,
            lockfile,
            name,
            close_signal: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn run_loop(&self) -> anyhow::Result<()> {
        loop {
            match self.try_connect().await {
                Ok(_) => {}
                Err(e) => log::warn!("[{}] WS error: {:?}", self.name, e),
            }

            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }

    async fn try_connect(&self) -> anyhow::Result<()> {
        let (tx, rx) = oneshot::channel();
        *self.close_signal.lock().unwrap() = Some(tx);

        let lf = match self.lockfile.read_lockfile() {
            Ok(v) => v,
            Err(_) => {
                return Ok(());
            }
        };

        let tcp = match tokio::net::TcpStream::connect(("127.0.0.1", lf.port)).await {
            Ok(v) => v,
            Err(_) => {
                return Ok(());
            }
        };

        let raw_url = format!("wss://127.0.0.1:{}/", lf.port);
        let mut req = raw_url.into_client_request()?;

        let token = base64::engine::general_purpose::STANDARD
            .encode(format!("riot:{}", lf.password));
        req.headers_mut()
            .insert("Authorization", format!("Basic {}", token).parse()?);

        let tls = self.build_insecure_tls()?;
        let connector = Connector::Rustls(tls);

        let (ws_stream, _) =
            tokio_tungstenite::client_async_tls_with_config(req, tcp, None, Some(connector)).await?;

        log::info!("[{}] Connected", self.name);

        let (mut write, mut read) = ws_stream.split();

        let subscribe_msg = serde_json::json!([5, "OnJsonApiEvent"]).to_string();
        write
            .send(tungstenite::Message::Text(subscribe_msg.into()))
            .await?;

        tokio::pin!(rx);

        loop {
            let next_msg = read.next();
            tokio::pin!(next_msg);

            match select(next_msg, &mut rx).await {
                Either::Left((msg, _)) => {
                    let msg = match msg {
                        Some(Ok(m)) => m,
                        _ => break,
                    };
                    if let tungstenite::Message::Text(txt) = msg {
                        self.handle_raw_message(txt.to_string());
                    }
                }
                Either::Right((_closed, _)) => {
                    log::warn!("[{}] WS force-closed", self.name);
                    break;
                }
            }
        }

        Ok(())
    }

    fn handle_raw_message(&self, msg: String) {
        use serde_json::Value;

        if let Ok(Value::Array(arr)) = serde_json::from_str::<Value>(&msg) {
            if arr.len() >= 3 {
                let frame = LcuSocketFrame::new(
                    arr[0].as_i64().unwrap_or(-1),
                    arr[1].as_str().unwrap_or("").to_string(),
                    arr[2].clone(),
                    self.name.clone(),
                );
                self.adapter.on_message(frame);
            }
        }
    }

    fn build_insecure_tls(&self) -> Result<Arc<rustls::ClientConfig>, rustls::Error> {
        use rustls::{ClientConfig, RootCertStore};

        let root = RootCertStore::empty();

        let mut config = ClientConfig::builder()
            .with_root_certificates(root)
            .with_no_client_auth();

        config.dangerous().set_certificate_verifier(verifier());

        Ok(Arc::new(config))
    }

    pub async fn force_close(&self) {
        if let Some(tx) = self.close_signal.lock().unwrap().take() {
            let _ = tx.send(());
        }
    }
}
