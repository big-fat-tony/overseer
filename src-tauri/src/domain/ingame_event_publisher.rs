use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};

use dashmap::DashMap;
use serde_json::Value;

use crate::domain::events::{EventType, LeagueEvent};
use crate::domain::ports::{
    IngameEventPublisherPort, IngameEventSubscriber, LeagueEventSubscriber,
};

use crate::adapters::outbound::ingame_api_client::IngameEventPoller;

pub struct IngameEventPublisher {
    poller: tokio::sync::Mutex<IngameEventPoller>,
    subscribers: Arc<DashMap<u64, Arc<dyn IngameEventSubscriber>>>,
    next_id: AtomicU64,
    active: AtomicBool,
}

impl IngameEventPublisher {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            poller: tokio::sync::Mutex::new(IngameEventPoller::new()),
            subscribers: Arc::new(DashMap::new()),
            next_id: AtomicU64::new(0),
            active: AtomicBool::new(false),
        })
    }

    /// Runs the polling loop inside the Tauri async runtime.
    pub fn start(self: Arc<Self>) {
        tauri::async_runtime::spawn(async move {
            loop {
                if self.active.load(Ordering::SeqCst) {
                    let mut poller = self.poller.lock().await;
                    let events = poller.poll_once().await;

                    for ev in events {
                        self.publish(&ev);
                    }
                }

                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            }
        });
    }
}

impl IngameEventPublisherPort for IngameEventPublisher {
    fn subscribe(&self, sub: Arc<dyn IngameEventSubscriber>) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.subscribers.insert(id, sub);
        id
    }

    fn unsubscribe(&self, id: u64) {
        self.subscribers.remove(&id);
    }

    fn publish(&self, event: &Value) {
        for entry in self.subscribers.iter() {
            let sub = entry.value().clone();
            let ev = event.clone();

            tauri::async_runtime::spawn(async move {
                sub.on_ingame_event(&ev).await;
            });
        }
    }
}

impl LeagueEventSubscriber for IngameEventPublisher {
    fn on_event(&self, event: &LeagueEvent) {
        if let EventType::GameflowSession = event.event_type {
            let phase = event
                .data
                .get("phase")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            match phase {
                "Loading" | "GameStart" | "InProgress" => self.active.store(true, Ordering::SeqCst),

                "PreEndOfGame" | "EndOfGame" | "WaitingForStats" => {
                    self.active.store(false, Ordering::SeqCst)
                }

                _ => {}
            }
        }
    }
}
