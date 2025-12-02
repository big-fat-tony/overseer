use std::collections::HashMap;
use std::sync::Arc;

use crate::domain::feature::{Feature, FeatureId};
use crate::domain::ports::{
    IngameEventPublisherPort, LeagueEventPublisherPort, LogPublisherPort, LcuApiPort,
    ChampionResolverPort, DataDragonApiPort, HovererPort, PickerPort, BannerPort,
};
use crate::domain::rune_page_manager::RunePageManager;

use crate::application::features::auto_pick_ban::feature::AutoPickBanFeature;
use crate::application::features::league_log::feature::LeagueEventLogFeature;
use crate::application::features::ingame_log::feature::IngameEventLogFeature;
use crate::application::features::match_ready::feature::MatchReadyFeature;
use crate::application::features::auto_pick_ban::champion_preferences::ChampionPreferences;
use crate::application::features::auto_pick_ban::priority_pick_strategy::PriorityPickStrategy;
use crate::application::features::auto_pick_ban::priority_ban_strategy::PriorityBanStrategy;
use crate::application::features::rune_picker::feature::RunePickerFeature;

use crate::adapters::outbound::data_dragon_champion_resolver::DataDragonChampionResolver;
use crate::adapters::outbound::lcu_champion_hoverer::ChampionHoverer;
use crate::adapters::outbound::lcu_champion_picker::ChampionPicker;
use crate::adapters::outbound::lcu_champion_banner::ChampionBanner;

use crate::domain::champ_select::ban_preferences::BanPreferences;
use crate::domain::delay_state::DelayState;

pub struct FeatureRegistry {
    map: HashMap<FeatureId, Arc<dyn Feature>>,
}

impl FeatureRegistry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        league_pub: Arc<dyn LeagueEventPublisherPort>,
        ingame_pub: Arc<dyn IngameEventPublisherPort>,
        log_pub: Arc<dyn LogPublisherPort>,
        lcu_api: Arc<dyn LcuApiPort>,
        data_dragon_api: Arc<dyn DataDragonApiPort>,
        delay: Arc<DelayState>,
        rune_pages: Arc<RunePageManager>,
    ) -> Self {
        let mut map = HashMap::<FeatureId, Arc<dyn Feature>>::new();

        let resolver: Arc<dyn ChampionResolverPort> =
            Arc::new(DataDragonChampionResolver::new(data_dragon_api.clone()));

        let pick_prefs = Arc::new(ChampionPreferences::new());
        let ban_prefs = Arc::new(BanPreferences::new());

        let pick_strategy = Arc::new(PriorityPickStrategy::new(pick_prefs.clone()));
        let ban_strategy = Arc::new(PriorityBanStrategy::new(ban_prefs.clone()));

        let hoverer: Arc<dyn HovererPort> =
            Arc::new(ChampionHoverer::new(lcu_api.clone(), pick_strategy.clone()));

        let picker: Arc<dyn PickerPort> =
            Arc::new(ChampionPicker::new(lcu_api.clone(), pick_strategy.clone()));

        let banner: Arc<dyn BannerPort> =
            Arc::new(ChampionBanner::new(lcu_api.clone(), ban_strategy.clone()));

        map.insert(
            FeatureId::LeagueLog,
            Arc::new(LeagueEventLogFeature::new(
                league_pub.clone(),
                log_pub.clone(),
            )),
        );

        map.insert(
            FeatureId::IngameLog,
            Arc::new(IngameEventLogFeature::new(
                ingame_pub.clone(),
                log_pub.clone(),
            )),
        );

        map.insert(
            FeatureId::MatchReady,
            Arc::new(MatchReadyFeature::new(
                league_pub.clone(),
                lcu_api.clone(),
                delay.clone(),
            )),
        );

        map.insert(
            FeatureId::RunePicker,
            Arc::new(RunePickerFeature::new(
                league_pub.clone(),
                lcu_api.clone(),
                delay.clone(),
                rune_pages.clone(),
                resolver.clone(),
            )),
        );

        map.insert(
            FeatureId::AutoPickBan,
            Arc::new(AutoPickBanFeature::new(
                league_pub.clone(),
                hoverer.clone(),
                picker.clone(),
                banner.clone(),
                pick_prefs.clone(),
                ban_prefs.clone(),
                resolver.clone(),
            )),
        );

        Self { map }
    }

    pub fn get(&self, id: &FeatureId) -> Option<Arc<dyn Feature>> {
        self.map.get(id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<dyn Feature>> {
        self.map.values().cloned().collect()
    }
}
