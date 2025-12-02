use std::sync::Arc;
use crate::application::features::rune_picker::subscriber::RunePickerSubscriber;
use crate::domain::feature::{Feature, FeatureId};
use crate::domain::ports::{LeagueEventPublisherPort, LeagueEventSubscriber, LcuApiPort, ChampionResolverPort};
use crate::domain::delay_state::DelayState;
use crate::domain::rune_page_manager::RunePageManager;

pub struct RunePickerFeature {
    league_pub: Arc<dyn LeagueEventPublisherPort>,
    subscriber: Arc<dyn LeagueEventSubscriber>,
    delay: Arc<DelayState>,
}

impl RunePickerFeature {
    pub fn new(
        league_pub: Arc<dyn LeagueEventPublisherPort>,
        api: Arc<dyn LcuApiPort>,
        delay: Arc<DelayState>,
        pages: Arc<RunePageManager>,
        resolver: Arc<dyn ChampionResolverPort>,
    ) -> Self {
        let subscriber = RunePickerSubscriber::new(api, delay.clone(), pages, resolver);
        Self {
            league_pub,
            subscriber,
            delay,
        }
    }
}

impl Feature for RunePickerFeature {
    fn id(&self) -> FeatureId {
        FeatureId::RunePicker
    }

    fn display_name(&self) -> String {
        "Rune Picker".into()
    }

    fn enable(&self) {
        self.league_pub.subscribe(self.subscriber.clone());
    }

    fn disable(&self) {
        self.league_pub.unsubscribe(&self.subscriber);
    }

    fn configure(&self, settings: serde_json::Value) {
        if let Some(v) = settings.get("delayMs").and_then(|v| v.as_u64()) {
            self.delay.set(v);
        }
    }
}
