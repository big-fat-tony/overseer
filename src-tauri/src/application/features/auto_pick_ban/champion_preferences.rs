use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::ports::ChampionResolverPort;
use log::info;

pub struct ChampionPreferences {
    data: RwLock<HashMap<String, Vec<i32>>>,
}

impl ChampionPreferences {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_for_role(&self, role: &str) -> Vec<i32> {
        let g = self.data.read().await;
        g.get(&role.to_lowercase()).cloned().unwrap_or_default()
    }

    pub async fn resolve_from_names(
        &self,
        role_map: HashMap<String, Vec<String>>,
        resolver: Arc<dyn ChampionResolverPort>,
    ) {
        let mut resolved: HashMap<String, Vec<i32>> = HashMap::new();

        for (role, names) in role_map {
            let mut list = Vec::new();

            for name in names {
                if let Some(id) = resolver.resolve_id(&name).await {
                    list.push(id);
                } else {
                    info!("⚠ Unknown champion in preferences: {}", name);
                }
            }

            resolved.insert(role.to_lowercase(), list);
        }

        let mut g = self.data.write().await;
        *g = resolved;
    }

    pub async fn set_role(&self, role: &str, champs: Vec<i32>) {
        let mut g = self.data.write().await;
        g.insert(role.to_lowercase(), champs);
    }
}
