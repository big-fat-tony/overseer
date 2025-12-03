use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use async_trait::async_trait;
use log::info;
use serde::Deserialize;

use crate::domain::ports::{ChampionResolverPort, DataDragonApiPort};

#[derive(Debug, Deserialize)]
struct ChampionJsonData {
    #[serde(rename = "key")]
    key: String,
    name: String,
}


pub struct DataDragonChampionResolver {
    api: Arc<dyn DataDragonApiPort>,
    version: Arc<RwLock<Option<String>>>,
    name_to_id: Arc<RwLock<HashMap<String, i32>>>,
    id_to_name: Arc<RwLock<HashMap<i32, String>>>,
    champions_cache: Arc<RwLock<Option<Vec<(String, i32)>>>>,
}

impl Clone for DataDragonChampionResolver {
    fn clone(&self) -> Self {
        Self {
            api: Arc::clone(&self.api),
            version: Arc::clone(&self.version),
            name_to_id: Arc::clone(&self.name_to_id),
            id_to_name: Arc::clone(&self.id_to_name),
            champions_cache: Arc::clone(&self.champions_cache),
        }
    }
}

impl DataDragonChampionResolver {
    pub fn new(api: Arc<dyn DataDragonApiPort>) -> Self {
        Self {
            api,
            version: Arc::new(RwLock::new(None)),
            name_to_id: Arc::new(RwLock::new(HashMap::new())),
            id_to_name: Arc::new(RwLock::new(HashMap::new())),
            champions_cache: Arc::new(RwLock::new(None)),
        }
    }

    async fn ensure_version(&self) -> String {
        {
            let guard = self.version.read().await;
            if let Some(v) = &*guard {
                return v.clone();
            }
        }

        let versions = self.api.get_versions().await.unwrap_or_else(|e| {
            info!("⚠️ Failed to fetch versions: {}", e);
            vec!["latest".into()]
        });

        let version = versions.first().cloned().unwrap_or_else(|| "latest".into());

        let mut guard = self.version.write().await;
        *guard = Some(version.clone());

        version
    }

    pub(crate) async fn fetch_champions_once(&self) -> Vec<(String, i32)> {
        {
            let guard = self.champions_cache.read().await;
            if let Some(data) = &*guard {
                return data.clone();
            }
        }

        let version = self.ensure_version().await;

        let json = match self.api.get_champions_json(&version).await {
            Ok(v) => v,
            Err(e) => {
                info!("⚠️ Failed to fetch champions.json: {}", e);
                return vec![];
            }
        };

        let mut output = Vec::new();

        let Some(map) = json.get("data").and_then(|v| v.as_object()) else {
            info!("⚠️ champions.json missing `data` field");
            return output;
        };

        for champ_info in map.values() {
            let parsed: ChampionJsonData = match serde_json::from_value(champ_info.clone()) {
                Ok(v) => v,
                Err(e) => {
                    info!("⚠️ Failed to parse champion entry: {}", e);
                    continue;
                }
            };

            let id = match parsed.key.parse::<i32>() {
                Ok(v) => v,
                Err(e) => {
                    info!("⚠️ Invalid champion key {}: {}", parsed.key, e);
                    continue;
                }
            };

            output.push((parsed.name, id));
        }

        let mut guard = self.champions_cache.write().await;
        *guard = Some(output.clone());

        output
    }
}

#[async_trait]
impl ChampionResolverPort for DataDragonChampionResolver {
    async fn refresh_cache(&self) {
        let champs = self.fetch_champions_once().await;

        let mut n2i = self.name_to_id.write().await;
        let mut i2n = self.id_to_name.write().await;

        n2i.clear();
        i2n.clear();

        for (name, id) in champs {
            n2i.insert(name.to_lowercase(), id);
            i2n.insert(id, name);
        }
    }

    async fn resolve_id(&self, name: &str) -> Option<i32> {
        {
            let guard = self.name_to_id.read().await;
            if !guard.is_empty() {
                return guard.get(&name.to_lowercase()).cloned();
            }
        }

        self.refresh_cache().await;

        let guard = self.name_to_id.read().await;
        guard.get(&name.to_lowercase()).cloned()
    }

    async fn resolve_name(&self, champ_id: i32) -> Option<String> {
        {
            let guard = self.id_to_name.read().await;
            if !guard.is_empty() {
                return guard.get(&champ_id).cloned();
            }
        }

        self.refresh_cache().await;

        let guard = self.id_to_name.read().await;
        guard.get(&champ_id).cloned()
    }
}
