use std::sync::{Arc, RwLock};
use tokio::time::{sleep, Duration};

use crate::adapters::inbound::lcu_websocket_adapter::LcuWebSocketAdapter;
use crate::adapters::inbound::lcu_websocket_client::LcuWebSocketClient;
use crate::adapters::inbound::league_lockfile_provider::LeagueLockfileProvider;
use crate::domain::ports::{LeagueEventPublisherPort, LockfilePort};

pub struct LeagueLifetimeManager {
    league_pub: Arc<dyn LeagueEventPublisherPort>,
    ws: RwLock<Option<Arc<LcuWebSocketClient>>>,
    lockfile: Arc<LeagueLockfileProvider>,
}

impl LeagueLifetimeManager {
    pub fn new(league_pub: Arc<dyn LeagueEventPublisherPort>) -> Self {
        Self {
            league_pub,
            ws: RwLock::new(None),
            lockfile: Arc::new(LeagueLockfileProvider::new(None)),
        }
    }

    pub async fn run(self: Arc<Self>) {
        loop {
            if self.lockfile.read_lockfile().is_ok() {
                self.start().await;
                self.wait_until_closed().await;
                self.stop().await;
            }
            sleep(Duration::from_secs(1)).await;
        }
    }

    async fn start(&self) {
        let adapter = LcuWebSocketAdapter::new(self.league_pub.clone());
        let ws = Arc::new(LcuWebSocketClient::new(
            adapter,
            Box::new(LeagueLockfileProvider::new(None)),
            "overseer-lcu".into(),
        ));

        {
            let mut guard = self.ws.write().unwrap();
            *guard = Some(ws.clone());
        }

        tokio::spawn(async move {
            let _ = ws.run_loop().await;
        });
    }

    async fn stop(&self) {
        let ws_opt = {
            let mut guard = self.ws.write().unwrap();
            guard.take()
        };

        if let Some(ws) = ws_opt {
            ws.force_close().await;
        }
    }

    async fn wait_until_closed(&self) {
        loop {
            if self.lockfile.read_lockfile().is_err() {
                break;
            }
            sleep(Duration::from_secs(1)).await;
        }
    }
}
