use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;
use crate::application::features::auto_pick_ban::champion_preferences::ChampionPreferences;
use crate::domain::feature::{Feature, FeatureId};
use crate::domain::ports::{
    HovererPort,
    PickerPort,
    BannerPort,
    ChampionResolverPort,
    LeagueEventPublisherPort,
    LeagueEventSubscriber,
};

use crate::application::features::auto_pick_ban::subscriber::AutoPickBanSubscriber;
use crate::domain::champ_select::ban_preferences::BanPreferences;

pub struct AutoPickBanFeature {
    league_pub: Arc<dyn LeagueEventPublisherPort>,
    subscriber: Arc<dyn LeagueEventSubscriber>,

    hoverer: Arc<dyn HovererPort>,
    picker: Arc<dyn PickerPort>,
    banner: Arc<dyn BannerPort>,

    pick_prefs: Arc<ChampionPreferences>,
    ban_prefs: Arc<BanPreferences>,
    resolver: Arc<dyn ChampionResolverPort>,
}

impl AutoPickBanFeature {
    pub fn new(
        league_pub: Arc<dyn LeagueEventPublisherPort>,
        hoverer: Arc<dyn HovererPort>,
        picker: Arc<dyn PickerPort>,
        banner: Arc<dyn BannerPort>,
        pick_prefs: Arc<ChampionPreferences>,
        ban_prefs: Arc<BanPreferences>,
        resolver: Arc<dyn ChampionResolverPort>,
    ) -> Self {
        let subscriber = AutoPickBanSubscriber::new(
            hoverer.clone(),
            picker.clone(),
            banner.clone(),
        );

        Self {
            league_pub,
            subscriber,
            hoverer,
            picker,
            banner,
            pick_prefs,
            ban_prefs,
            resolver,
        }
    }

    fn apply_pick_settings(&self, val: &Value) {
        let Some(obj) = val.as_object() else {
            return;
        };

        let mut role_to_names = HashMap::<String, Vec<String>>::new();

        for (role, arr) in obj {
            let names = arr
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|v| v.as_str().map(|x| x.to_string()))
                .collect::<Vec<_>>();

            role_to_names.insert(role.clone(), names);
        }

        let prefs = self.pick_prefs.clone();
        let resolver = self.resolver.clone();

        tauri::async_runtime::spawn(async move {
            prefs.resolve_from_names(role_to_names, resolver).await;
        });
    }

    fn apply_ban_settings(&self, val: &Value) {
        let Some(obj) = val.as_object() else {
            return;
        };

        let mut role_to_names = HashMap::<String, Vec<String>>::new();

        for (role, arr) in obj {
            let names = arr
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|v| v.as_str().map(|x| x.to_string()))
                .collect::<Vec<_>>();

            role_to_names.insert(role.clone(), names);
        }

        let prefs = self.ban_prefs.clone();
        let resolver = self.resolver.clone();

        tauri::async_runtime::spawn(async move {
            prefs.resolve_from_names(role_to_names, resolver).await;
        });
    }
}

impl Feature for AutoPickBanFeature {
    fn id(&self) -> FeatureId {
        FeatureId::AutoPickBan
    }

    fn display_name(&self) -> String {
        "Auto Pick/Ban".into()
    }

    fn enable(&self) {
        self.league_pub.subscribe(self.subscriber.clone());
    }

    fn disable(&self) {
        self.league_pub.unsubscribe(&self.subscriber);
    }

    fn configure(&self, settings: Value) {
        if let Some(p) = settings.get("pickPreferences") {
            self.apply_pick_settings(p);
        }

        if let Some(b) = settings.get("banPreferences") {
            self.apply_ban_settings(b);
        }
    }
}
