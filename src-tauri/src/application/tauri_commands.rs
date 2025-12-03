use crate::adapters::outbound::data_dragon_champion_resolver::DataDragonChampionResolver;
use crate::domain::feature::{FeatureId, FeatureInfo};
use crate::domain::feature_manager::FeatureManager;
use crate::domain::rune_page::RunePage;
use crate::domain::rune_page_manager::RunePageManager;
use serde_json::Value;
use std::str::FromStr;
use std::sync::Arc;
use tauri::{AppHandle, State};
use tauri::ipc::Channel;
use tauri_plugin_updater::UpdaterExt;
use crate::application::dev_flag::DevFlag;

#[tauri::command]
pub fn list_features(state: State<Arc<FeatureManager>>) -> Vec<FeatureInfo> {
    log::info!("Listing features");
    state.get_features()
}

#[tauri::command]
pub fn enable_feature(id: String, state: State<Arc<FeatureManager>>) -> Result<(), String> {
    log::info!("Enable feature {}", id);
    state.enable_by_id(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn disable_feature(id: String, state: State<Arc<FeatureManager>>) -> Result<(), String> {
    log::info!("Disable feature {}", id);
    state.disable_by_id(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_feature_setting(
    state: State<Arc<FeatureManager>>,
    feature_id: String,
    key: String,
    value: Value,
) -> Result<(), String> {
    let id = FeatureId::from_str(&feature_id).map_err(|e| e.to_string())?;
    state
        .set_feature_setting(id, &key, value)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_feature_settings(
    state: State<Arc<FeatureManager>>,
    feature_id: String,
) -> Result<Value, String> {
    let id = FeatureId::from_str(&feature_id).map_err(|e| e.to_string())?;
    state.get_feature_settings(id).map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn list_champions(
    resolver: State<'_, DataDragonChampionResolver>,
) -> Result<Vec<(String, i32)>, String> {
    log::info!("Listing champions");
    Ok(resolver.fetch_champions_once().await)
}

#[tauri::command]
pub fn save_rune_page(state: State<Arc<RunePageManager>>, page: RunePage) -> Result<(), String> {
    log::info!("Saving rune page {}", page.name);
    state.save(page)
}

#[tauri::command]
pub fn list_rune_pages(state: State<Arc<RunePageManager>>) -> Vec<RunePage> {
    log::info!("Listing rune pages");
    state.list()
}

#[tauri::command]
pub fn delete_rune_page(state: State<Arc<RunePageManager>>, id: String) -> Result<(), String> {
    log::info!("Deleting rune page {}", id);
    state.delete(&id)
}

#[derive(Clone, serde::Serialize)]
#[serde(tag = "event", content = "data")]
pub enum UpdateEvent {
    Started { content_length: Option<u64> },
    Progress { chunk_length: usize },
    Finished,
}

#[derive(serde::Serialize)]
pub struct UpdateInfo {
    pub current: String,
    pub latest: String,
}

#[tauri::command]
pub async fn check_update(
    app: AppHandle,
    dev: State<'_, DevFlag>,
) -> Result<Option<UpdateInfo>, String> {
    if dev.is_dev {
        return Ok(None);
    }

    let current = app.package_info().version.to_string();
    let updater = app.updater().map_err(|e| e.to_string())?;
    let result = updater.check().await.map_err(|e| e.to_string())?;

    Ok(result.map(|u| UpdateInfo {
        current,
        latest: u.version,
    }))
}

#[tauri::command]
pub async fn install_update(
    app: AppHandle,
    on_event: Channel<UpdateEvent>,
) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    let update = updater.check().await.map_err(|e| e.to_string())?;

    let Some(update) = update else {
        return Err("no update available".into());
    };

    let mut started = false;

    update
        .download_and_install(
            |chunk, total| {
                if !started {
                    let _ = on_event.send(UpdateEvent::Started { content_length: total });
                    started = true;
                }
                let _ = on_event.send(UpdateEvent::Progress { chunk_length: chunk });
            },
            || {
                let _ = on_event.send(UpdateEvent::Finished);
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_current_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}