use crate::domain::events::{EventType, LeagueEvent};
use crate::domain::lcu_socket_frame::LcuSocketFrame;
use serde_json::Value;

/// Default parser used for all event types unless a special one is implemented.
pub struct EventRegistry;

impl EventRegistry {
    pub fn parse(event_type: EventType, frame: &LcuSocketFrame) -> LeagueEvent {
        let payload = frame.payload.clone();

        // Extract the "data" field from LCU structure
        let data = payload.get("data").cloned().unwrap_or(Value::Null);

        LeagueEvent {
            event_type,
            data,
            raw: frame.clone(),
        }
    }

    pub fn map_uri(uri: &str) -> EventType {
        match uri {
            "/lol-gameflow/v1/session" => EventType::GameflowSession,
            "/lol-gameflow/v1/gameflow-phase" => EventType::GameflowPhase,
            "/lol-champ-select/v1/session" => EventType::ChampSelectSession,
            "/riot-messaging-service/v1/message/parties/v1/notifications" => {
                EventType::PartiesNotification
            }
            "/riot-messaging-service/v1/message/teambuilder/v1/tbdGameDtoV1" => {
                EventType::TeambuilderTBDGame
            }
            _ => EventType::Unknown,
        }
    }
}
