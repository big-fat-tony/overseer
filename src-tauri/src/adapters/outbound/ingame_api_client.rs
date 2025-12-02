use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

pub struct IngameApiClient {
    client: Client,
    base_url: String,
}

impl IngameApiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_millis(200))
            .build()
            .expect("failed to build reqwest client");

        Self {
            client,
            base_url: "https://127.0.0.1:2999".into(),
        }
    }

    pub async fn fetch_events(&self, from_id: u32) -> anyhow::Result<Vec<Value>> {
        let url = format!(
            "{}/GetLiveclientdataEventdata?eventID={}",
            self.base_url, from_id
        );

        let resp = self.client.get(url).send().await?;
        let json: Value = resp.json().await?;
        let events = json
            .get("Events")
            .or_else(|| json.get("events"))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        Ok(events)
    }
}

pub struct IngameEventPoller {
    api: IngameApiClient,
    next_id: u32,
}

impl IngameEventPoller {
    pub fn new() -> Self {
        Self {
            api: IngameApiClient::new(),
            next_id: 0,
        }
    }

    /// Poll once and return any new events.
    /// The caller publishes them.
    pub async fn poll_once(&mut self) -> Vec<Value> {
        match self.api.fetch_events(self.next_id).await {
            Ok(events) => {
                if let Some(max_id) = events
                    .iter()
                    .filter_map(|ev| {
                        ev.get("EventID")
                            .or_else(|| ev.get("eventID"))
                            .and_then(|v| v.as_u64())
                    })
                    .max()
                {
                    // Detect new game or wraparound
                    if max_id < self.next_id as u64 {
                        self.next_id = 0;
                    }

                    self.next_id = (max_id as u32).saturating_add(1);
                }

                events
            }

            Err(e) => {
                log::debug!("Ingame API error: {:?}", e);
                Vec::new()
            }
        }
    }
}
