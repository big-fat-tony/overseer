use std::sync::Arc;


use crate::domain::events::LeagueEvent;
use crate::domain::ports::{LeagueEventSubscriber, LogPublisherPort};
use crate::domain::log_entry::LogEntry;

pub struct LeagueLogSubscriber {
    log_pub: Arc<dyn LogPublisherPort>,
}

impl LeagueLogSubscriber {
    pub fn new(log_pub: Arc<dyn LogPublisherPort>) -> Arc<Self> {
        Arc::new(Self { log_pub })
    }
}

impl LeagueEventSubscriber for LeagueLogSubscriber {
    fn on_event(&self, event: &LeagueEvent) {
        let json = serde_json::json!({
            "event_type": format!("{:?}", event.event_type),
            "data": event.data,
        });

        let entry = LogEntry {
            source: "league".into(),
            payload: json,
        };

        let _ = self.log_pub.publish(entry);
    }
}
