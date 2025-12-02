use std::sync::Arc;
use serde_json::Value;
use async_trait::async_trait;
use crate::domain::ports::{IngameEventSubscriber, LogPublisherPort};
use crate::domain::log_entry::LogEntry;

pub struct IngameEventLogSubscriber {
    log_pub: Arc<dyn LogPublisherPort>,
}

impl IngameEventLogSubscriber {
    pub fn new(log_pub: Arc<dyn LogPublisherPort>) -> Arc<Self> {
        Arc::new(Self { log_pub })
    }
}

#[async_trait]
impl IngameEventSubscriber for IngameEventLogSubscriber {
    async fn on_ingame_event(&self, event: &Value) {
        let entry = LogEntry {
            source: "ingame".into(),
            payload: event.clone(),
        };

        let _ = self.log_pub.publish(entry);
    }
}
