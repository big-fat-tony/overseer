use crate::domain::lcu_socket_frame::LcuSocketFrame;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    All,
    ChampSelectSession,
    GameflowPhase,
    GameflowSession,
    PartiesNotification,
    TeambuilderTBDGame,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct LeagueEvent {
    pub event_type: EventType,
    pub data: Value,
    pub raw: LcuSocketFrame,
}
