use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use crate::domain::log_entry::LogEntry;
use crate::domain::ports::LogPublisherPort;

pub struct TauriLogPublisherAdapter {
    app: AppHandle,
}

impl TauriLogPublisherAdapter {
    pub fn new(app: AppHandle) -> Arc<Self> {
        Arc::new(Self { app })
    }
}

impl LogPublisherPort for TauriLogPublisherAdapter {
    fn publish(&self, entry: LogEntry) {
        let app = self.app.clone();
        let payload = entry.clone();

        tauri::async_runtime::spawn(async move {
            let _ = app.emit("log-entry", payload);
        });
    }
}
