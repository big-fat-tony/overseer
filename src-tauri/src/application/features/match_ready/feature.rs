use std::sync::Arc;

use crate::domain::delay_state::DelayState;
use crate::domain::feature::{Feature, FeatureId};
use crate::domain::ports::{LcuApiPort, LeagueEventPublisherPort, LeagueEventSubscriber};

use crate::application::features::match_ready::subscriber::MatchReadySubscriber;

pub struct MatchReadyFeature {
    league_pub: Arc<dyn LeagueEventPublisherPort>,
    subscriber: Arc<dyn LeagueEventSubscriber>,
    delay: Arc<DelayState>,
}

impl MatchReadyFeature {
    pub fn new(
        league_pub: Arc<dyn LeagueEventPublisherPort>,
        api: Arc<dyn LcuApiPort>,
        delay: Arc<DelayState>,
    ) -> Self {
        let subscriber = MatchReadySubscriber::new(api, delay.clone());

        Self {
            league_pub,
            subscriber,
            delay,
        }
    }
}

impl Feature for MatchReadyFeature {
    fn id(&self) -> FeatureId {
        FeatureId::MatchReady
    }

    fn display_name(&self) -> String {
        "Match Ready Auto-Accept".into()
    }

    fn enable(&self) {
        self.league_pub.subscribe(self.subscriber.clone());
        log::debug!("Match Ready Auto-Accept enabled");
    }

    fn disable(&self) {
        self.league_pub.unsubscribe(&self.subscriber);
        log::debug!("Match Ready Auto-Accept disabled");
    }

    fn configure(&self, settings: serde_json::Value) {
        if let Some(v) = settings.get("delayMs").and_then(|v| v.as_u64()) {
            log::info!("Match Ready Auto-Accept configured with {} ms", v);
            self.delay.set(v);
        }
    }
}
