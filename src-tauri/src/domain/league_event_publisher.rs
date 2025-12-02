use crate::domain::events::LeagueEvent;
pub(crate) use crate::domain::ports::LeagueEventPublisherPort;
use crate::domain::ports::LeagueEventSubscriber;

use std::sync::{Arc, Mutex};

pub struct LeagueEventPublisher {
    subscribers: Mutex<Vec<Arc<dyn LeagueEventSubscriber>>>,
}

impl LeagueEventPublisher {
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(Vec::new()),
        }
    }
}

impl LeagueEventPublisherPort for LeagueEventPublisher {
    fn subscribe(&self, subscriber: Arc<dyn LeagueEventSubscriber>) {
        self.subscribers.lock().unwrap().push(subscriber);
    }

    fn unsubscribe(&self, target: &Arc<dyn LeagueEventSubscriber>) {
        self.subscribers
            .lock()
            .unwrap()
            .retain(|sub| !Arc::ptr_eq(sub, target));
    }

    fn publish(&self, event: &LeagueEvent) {
        for sub in self.subscribers.lock().unwrap().iter() {
            sub.on_event(event);
        }
    }
}
