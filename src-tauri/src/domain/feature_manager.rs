use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tauri::Wry;
use tauri_plugin_store::Store;

use crate::domain::feature::{Feature, FeatureId, FeatureInfo};
use crate::domain::feature_registry::FeatureRegistry;

pub struct FeatureManager {
    registry: FeatureRegistry,
    active: Mutex<HashMap<FeatureId, Arc<dyn Feature>>>,
    store: Arc<Store<Wry>>,
}

impl FeatureManager {
    pub fn new(registry: FeatureRegistry, store: Arc<Store<Wry>>) -> Self {
        let mgr = Self {
            registry,
            active: Mutex::new(HashMap::new()),
            store,
        };

        mgr.restore_state();
        mgr
    }

    fn restore_state(&self) {
        let enabled = self
            .store
            .get("enabled")
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default();

        let mut active = self.active.lock().unwrap();

        for entry in enabled {
            if let Some(id) = entry.as_str() {
                if let Ok(feature_id) = FeatureId::from_str(id) {
                    self.enable(feature_id, &mut active);
                }
            }
        }
    }

    fn save_state(&self) {
        let active = self.active.lock().unwrap();
        let list: Vec<Value> = active
            .keys()
            .map(|k| Value::String(k.to_string()))
            .collect();
        self.store.set("enabled", Value::Array(list));
    }

    pub fn is_enabled(&self, id: &str) -> bool {
        let feature_id = match FeatureId::from_str(id) {
            Ok(v) => v,
            Err(_) => return false,
        };

        let active = self.active.lock().unwrap();
        active.contains_key(&feature_id)
    }

    pub fn enable_by_id(&self, id: &str) -> anyhow::Result<()> {
        let feature_id = FeatureId::from_str(id)?;
        let mut active = self.active.lock().unwrap();
        self.enable(feature_id, &mut active);
        drop(active);
        self.save_state();
        Ok(())
    }

    fn enable(&self, id: FeatureId, active: &mut HashMap<FeatureId, Arc<dyn Feature>>) {
        if active.contains_key(&id) {
            return;
        }

        if let Some(feature) = self.registry.get(&id) {
            let settings = self.get_feature_settings(id).unwrap_or(Value::Null);
            feature.configure(settings);
            feature.enable();
            active.insert(id, feature);
        }
    }

    pub fn apply_setting_to_active(&self, id: FeatureId, key: &str, value: Value) {
        let active = self.active.lock().unwrap();

        if let Some(feature) = active.get(&id) {
            // Build a fake mini-settings JSON for configure()
            let mut map = serde_json::Map::new();
            map.insert(key.to_string(), value);
            let settings = Value::Object(map);

            feature.configure(settings);
        }
    }

    pub fn disable_by_id(&self, id: &str) -> anyhow::Result<()> {
        let feature_id = FeatureId::from_str(id)?;
        self.disable(&feature_id);
        self.save_state();
        Ok(())
    }

    pub fn disable(&self, id: &FeatureId) {
        let mut active = self.active.lock().unwrap();

        if let Some(feature) = active.remove(id) {
            feature.disable();
        }
    }

    pub fn get_features(&self) -> Vec<FeatureInfo> {
        self.registry
            .all()
            .iter()
            .map(|f| FeatureInfo {
                id: f.id().to_string(),
                name: f.display_name(),
                enabled: self.is_enabled(&f.id().to_string()),
            })
            .collect()
    }

    pub fn set_feature_setting(
        &self,
        id: FeatureId,
        key: &str,
        value: Value,
    ) -> anyhow::Result<()> {
        let store_key = format!("feature.{}.{}", id, key);
        log::info!("set_feature_setting({}, {})", id, store_key);

        self.store.set(store_key, value.clone()); // save persistently
        self.apply_setting_to_active(id, key, value); // apply immediately

        Ok(())
    }

    pub fn get_feature_settings(&self, id: FeatureId) -> anyhow::Result<Value> {
        let prefix = format!("feature.{}.", id);
        log::info!("get_feature_settings({})", prefix);

        let mut map = serde_json::Map::new();

        for (k, v) in self.store.entries().iter() {
            if k.starts_with(&prefix) {
                let key = k.trim_start_matches(&prefix).to_string();
                map.insert(key, v.clone());
            }
        }

        Ok(Value::Object(map))
    }
}
