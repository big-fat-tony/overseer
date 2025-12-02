pub mod adapters;
pub mod application;
pub mod domain;

use crate::application::league_lifetime_manager::LeagueLifetimeManager;
use crate::application::tauri_commands::*;
use crate::domain::feature_manager::FeatureManager;
use crate::domain::feature_registry::FeatureRegistry;
use crate::domain::ports::LeagueEventPublisherPort;
use crate::domain::rune_page_manager::RunePageManager;
use std::sync::Arc;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_store::StoreExt;

use crate::domain::ingame_event_publisher::IngameEventPublisher;
use crate::domain::league_event_publisher::LeagueEventPublisher;

use crate::adapters::outbound::data_dragon_api::DataDragonApiAdapter;
use crate::adapters::outbound::data_dragon_champion_resolver::DataDragonChampionResolver;
use crate::adapters::outbound::lcu_api::LcuApiAdapter;
use crate::adapters::outbound::tauri_log_publisher::TauriLogPublisherAdapter;

use crate::adapters::inbound::league_lockfile_provider::LeagueLockfileProvider;
use crate::domain::delay_state::DelayState;
use crate::domain::ports::LockfilePort;

fn init_core() -> (
    Arc<LeagueEventPublisher>,
    Arc<IngameEventPublisher>,
    Arc<dyn crate::domain::ports::LcuApiPort>,
) {
    let league_pub = Arc::new(LeagueEventPublisher::new());
    let ingame_pub = IngameEventPublisher::new();
    league_pub.subscribe(ingame_pub.clone());

    let lockfile: Arc<dyn LockfilePort> = Arc::new(LeagueLockfileProvider::new(None));

    let lcu_api = Arc::new(LcuApiAdapter::new(lockfile.clone()).expect("Failed to init LCU API"))
        as Arc<dyn crate::domain::ports::LcuApiPort>;

    (league_pub, ingame_pub, lcu_api)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (league_pub, ingame_pub, lcu_api) = init_core();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            list_features,
            enable_feature,
            disable_feature,
            get_feature_settings,
            set_feature_setting,
            list_champions,
            save_rune_page,
            list_rune_pages,
            delete_rune_page
        ])
        .setup(move |app| {
            let lifetime = Arc::new(LeagueLifetimeManager::new(league_pub.clone()));
            app.manage(lifetime.clone());

            tauri::async_runtime::spawn({
                let lm = lifetime.clone();
                async move { lm.run().await }
            });

            ingame_pub.clone().start();

            let store = app.store("overseer-test.json")?;
            let rpm = Arc::new(RunePageManager::new(store.clone()));
            let delay = Arc::new(DelayState::new());
            let dd_api = Arc::new(DataDragonApiAdapter::new());
            let resolver = DataDragonChampionResolver::new(dd_api.clone());
            let log_pub = TauriLogPublisherAdapter::new(app.handle().clone());

            let registry = FeatureRegistry::new(
                league_pub.clone(),
                ingame_pub.clone(),
                log_pub.clone(),
                lcu_api.clone(),
                dd_api.clone(),
                delay.clone(),
                rpm.clone(),
            );

            let feature_manager = Arc::new(FeatureManager::new(registry, store.clone()));

            app.manage(log_pub);
            app.manage(feature_manager);
            app.manage(league_pub);
            app.manage(ingame_pub);
            app.manage(delay);
            app.manage(dd_api);
            app.manage(resolver);
            app.manage(rpm);

            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap();
}
