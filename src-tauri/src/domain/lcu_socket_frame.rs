use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct LcuSocketFrame {
    pub opcode: i64,
    pub lcu_event_type: String,
    pub payload: Value,
    pub name: String,
    pub ts: DateTime<Utc>,
}

impl LcuSocketFrame {
    pub fn new(opcode: i64, event_type: String, payload: Value, name: String) -> Self {
        Self {
            opcode,
            lcu_event_type: event_type,
            payload,
            name,
            ts: Utc::now(),
        }
    }
}
