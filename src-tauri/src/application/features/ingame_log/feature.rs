use std::sync::{Arc, Mutex};
use crate::domain::feature::{Feature, FeatureId};
use crate::domain::ports::{IngameEventPublisherPort, IngameEventSubscriber, LogPublisherPort};
use crate::application::features::ingame_log::subscriber::IngameEventLogSubscriber;

pub struct IngameEventLogFeature {
    ingame_pub: Arc<dyn IngameEventPublisherPort>,
    subscriber: Arc<dyn IngameEventSubscriber>,
    sub_id: Mutex<Option<u64>>,
}

impl IngameEventLogFeature {
    pub fn new(
        ingame_pub: Arc<dyn IngameEventPublisherPort>,
        log_pub: Arc<dyn LogPublisherPort>,
    ) -> Self {
        let subscriber: Arc<dyn IngameEventSubscriber> =
            IngameEventLogSubscriber::new(log_pub.clone());

        Self {
            ingame_pub,
            subscriber,
            sub_id: Mutex::new(None),
        }
    }
}

impl Feature for IngameEventLogFeature {
    fn id(&self) -> FeatureId {
        FeatureId::IngameLog
    }

    fn display_name(&self) -> String {
        "Ingame Event Log".to_string()
    }

    fn enable(&self) {
        let id = self.ingame_pub.subscribe(self.subscriber.clone());
        *self.sub_id.lock().unwrap() = Some(id);
        log::debug!("Ingame Event Log enabled");
    }

    fn disable(&self) {
        if let Some(id) = *self.sub_id.lock().unwrap() {
            self.ingame_pub.unsubscribe(id);
            log::debug!("Ingame Event Log disabled");
        }
    }
}
