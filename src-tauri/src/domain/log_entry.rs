use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct LogEntry {
    pub source: String, // "ingame" or "league"
    pub payload: serde_json::Value,
}
