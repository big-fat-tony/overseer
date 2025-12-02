use std::sync::Arc;

use crate::domain::lcu_socket_frame::LcuSocketFrame;
use crate::domain::league_event_publisher::LeagueEventPublisherPort;
use crate::domain::registry::EventRegistry;

pub struct LcuWebSocketAdapter {
    publisher: Arc<dyn LeagueEventPublisherPort>,
}

impl LcuWebSocketAdapter {
    pub fn new(publisher: Arc<dyn LeagueEventPublisherPort>) -> Self {
        Self { publisher }
    }

    pub fn on_message(&self, frame: LcuSocketFrame) {
        let uri = frame
            .payload
            .get("uri")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let event_type = EventRegistry::map_uri(uri);
        let event = EventRegistry::parse(event_type, &frame);

        self.publisher.publish(&event);
    }
}
