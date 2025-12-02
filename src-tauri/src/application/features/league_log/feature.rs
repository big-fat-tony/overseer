use std::sync::{Arc, Mutex};

use crate::domain::feature::{Feature, FeatureId};
use crate::domain::ports::{LeagueEventPublisherPort, LeagueEventSubscriber, LogPublisherPort};

use crate::application::features::league_log::subscriber::LeagueLogSubscriber;

pub struct LeagueEventLogFeature {
    league_pub: Arc<dyn LeagueEventPublisherPort>,
    subscriber: Arc<dyn LeagueEventSubscriber>,
    sub_ref: Mutex<Option<Arc<dyn LeagueEventSubscriber>>>,
}

impl LeagueEventLogFeature {
    pub fn new(
        league_pub: Arc<dyn LeagueEventPublisherPort>,
        log_pub: Arc<dyn LogPublisherPort>,
    ) -> Self {
        let subscriber: Arc<dyn LeagueEventSubscriber> =
            LeagueLogSubscriber::new(log_pub.clone());

        Self {
            league_pub,
            subscriber,
            sub_ref: Mutex::new(None),
        }
    }
}

impl Feature for LeagueEventLogFeature {
    fn id(&self) -> FeatureId {
        FeatureId::LeagueLog
    }

    fn display_name(&self) -> String {
        "League Event Log".to_string()
    }

    fn enable(&self) {
        self.league_pub.subscribe(self.subscriber.clone());
        *self.sub_ref.lock().unwrap() = Some(self.subscriber.clone());
        log::debug!("League Event Log enabled");
    }

    fn disable(&self) {
        if let Some(sub) = self.sub_ref.lock().unwrap().as_ref() {
            self.league_pub.unsubscribe(sub);
            log::debug!("League Event Log disabled");
        }
    }
}
