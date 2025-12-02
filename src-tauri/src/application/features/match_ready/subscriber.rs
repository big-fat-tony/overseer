use crate::domain::delay_state::DelayState;
use crate::domain::events::{EventType, LeagueEvent};
use crate::domain::ports::{LcuApiPort, LeagueEventSubscriber};
use log::info;
use serde_json::Value;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct MatchReadySubscriber {
    api: Arc<dyn LcuApiPort>,
    delay: Arc<DelayState>,
}

impl MatchReadySubscriber {
    pub fn new(api: Arc<dyn LcuApiPort>, delay: Arc<DelayState>) -> Arc<Self> {
        Arc::new(Self { api, delay })
    }

    fn accept_with_delay(&self) {
        let delay = self.delay.get();
        let api = Arc::clone(&self.api);
        let endpoint = "/lol-matchmaking/v1/ready-check/accept";

        info!("[ACCEPTING MATCH IN {} ms] POST {}", delay, endpoint);

        tokio::spawn(async move {
            sleep(Duration::from_millis(delay)).await;
            let res = api.post(endpoint, None).await;
            match res {
                Ok(_) => info!("[MATCH ACCEPTED] Accepted via {}", endpoint),
                Err(e) => info!("[MATCH ACCEPT ERROR] {:?}", e),
            }
        });
    }
}

impl LeagueEventSubscriber for MatchReadySubscriber {
    fn on_event(&self, event: &LeagueEvent) {
        if event.event_type != EventType::TeambuilderTBDGame {
            return;
        }

        let payload_raw = match event.data.get("payload") {
            Some(v) => v,
            None => return,
        };

        let payload: Value = match serde_json::from_str(payload_raw.as_str().unwrap_or("")) {
            Ok(v) => v,
            Err(e) => {
                info!("Failed to parse inner payload: {:?}", e);
                return;
            }
        };

        info!("Got new TBD event: {:?}", payload);

        let phase_name = payload
            .get("phaseName")
            .and_then(Value::as_str)
            .unwrap_or("");

        if phase_name != "AFK_CHECK" {
            return;
        }

        info!("[{}] AFK_CHECK detected", event.raw.name);

        let queue_id = payload
            .get("queueId")
            .and_then(Value::as_i64)
            .unwrap_or_default();

        if queue_id == 3140 {
            info!("[MatchReadySubscriber] Skipping auto-accept for Practice Tool");
            return;
        }

        info!("afk_check_state: {:?}", payload.get("afkCheckState"));

        self.accept_with_delay();
    }
}
