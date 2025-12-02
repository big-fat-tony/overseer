use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunePage {
    pub id: String,
    pub name: String,
    pub champions: Vec<String>,
    pub role: Option<String>,
    pub primary_tree_id: Option<i32>,
    pub primary_picks: HashMap<i32, Option<i32>>,
    pub secondary_tree_id: Option<i32>,
    pub secondary_picks: HashMap<i32, Option<i32>>,
    pub shards: HashMap<i32, Option<i32>>,
}

impl RunePage {
    pub fn collect_perk_ids(&self) -> Vec<i32> {
        let mut out = Vec::new();

        // Primary tree: 4 slots
        for slot in [0, 1, 2, 3] {
            if let Some(Some(id)) = self.primary_picks.get(&slot) {
                out.push(*id);
            }
        }

        // Secondary tree: 2 slots
        for slot in [0, 1] {
            if let Some(Some(id)) = self.secondary_picks.get(&slot) {
                out.push(*id);
            }
        }

        // Shards: 3 slots
        for slot in [0, 1, 2] {
            if let Some(Some(id)) = self.shards.get(&slot) {
                out.push(*id);
            }
        }

        out
    }
}
