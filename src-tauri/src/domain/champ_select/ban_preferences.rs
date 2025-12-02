use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::ports::ChampionResolverPort;
use log::info;

pub struct BanPreferences {
    data: RwLock<HashMap<String, Vec<i32>>>,
}

impl BanPreferences {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }
    pub async fn get_for_role(&self, role: &str) -> Vec<i32> {
        let guard = self.data.read().await;
        guard.get(&role.to_lowercase()).cloned().unwrap_or_default()
    }

    pub async fn resolve_from_names(
        &self,
        role_to_names: HashMap<String, Vec<String>>,
        resolver: Arc<dyn ChampionResolverPort>,
    ) {
        let mut resolved: HashMap<String, Vec<i32>> = HashMap::new();

        for (role, names) in role_to_names {
            let mut ids = Vec::new();

            for name in names {
                if let Some(id) = resolver.resolve_id(&name).await {
                    ids.push(id);
                } else {
                    info!("Unknown champion in ban preferences: {}", name);
                }
            }

            resolved.insert(role.to_lowercase(), ids);
        }

        let mut guard = self.data.write().await;
        *guard = resolved;
    }

    pub async fn set_for_role(&self, role: &str, champ_ids: Vec<i32>) {
        let mut guard = self.data.write().await;
        guard.insert(role.to_lowercase(), champ_ids);
    }
}
