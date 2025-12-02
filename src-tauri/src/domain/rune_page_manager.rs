use crate::domain::ports::LcuApiPort;
use crate::domain::rune_page::RunePage;
use serde_json::{json, Value};
use std::sync::Arc;
use tauri::Wry;
use tauri_plugin_store::Store;

pub struct RunePageManager {
    store: Arc<Store<Wry>>,
}

impl RunePageManager {
    pub fn new(store: Arc<Store<Wry>>) -> Self {
        Self { store }
    }

    fn load_pages(&self) -> Vec<RunePage> {
        self.store
            .get("pages")
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect()
    }

    fn save_pages(&self, pages: Vec<RunePage>) -> Result<(), String> {
        let json: Vec<Value> = pages
            .into_iter()
            .map(|p| serde_json::to_value(p).unwrap())
            .collect();

        self.store.set("pages", Value::Array(json));
        Ok(())
    }

    pub fn save(&self, page: RunePage) -> Result<(), String> {
        let mut pages = self.load_pages();
        let id = page.id.clone();

        if let Some(existing) = pages.iter_mut().find(|p| p.id == id) {
            *existing = page;
        } else {
            pages.push(page);
        }

        self.save_pages(pages)
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        let pages = self
            .load_pages()
            .into_iter()
            .filter(|p| p.id != id)
            .collect::<Vec<_>>();

        self.save_pages(pages)
    }

    pub fn list(&self) -> Vec<RunePage> {
        self.load_pages()
    }

    pub fn find_pages(&self, champion: &str, role: Option<&str>) -> Vec<RunePage> {
        let pages = self.load_pages();

        pages
            .into_iter()
            .filter(|p| p.champions.iter().any(|c| c == champion))
            .filter(|p| match role {
                Some(r) => p
                    .role
                    .as_deref()
                    .map(|s| s.eq_ignore_ascii_case(r))
                    .unwrap_or(false),
                None => true,
            })
            .collect()
    }

    pub fn find_best_page(&self, champion: &str, role: Option<&str>) -> Option<RunePage> {
        let pages = self.find_pages(champion, role);
        pages.into_iter().next()
    }

    async fn fetch_current(&self, api: &dyn LcuApiPort) -> Option<Value> {
        api.get("/lol-perks/v1/currentpage").await.ok()
    }

    fn sorted(v: &[i32]) -> Vec<i32> {
        let mut x = v.to_vec();
        x.sort();
        x
    }

    fn same_perks(want: &[i32], have: &[i32]) -> bool {
        Self::sorted(want) == Self::sorted(have)
    }

    async fn same_page(&self, page: &RunePage, api: &dyn LcuApiPort) -> bool {
        let cur = match self.fetch_current(api).await {
            Some(v) => v,
            None => return false,
        };

        let prim = cur
            .get("primaryStyleId")
            .and_then(|n| n.as_i64())
            .unwrap_or(0) as i32;
        let sec = cur.get("subStyleId").and_then(|n| n.as_i64()).unwrap_or(0) as i32;

        if prim != page.primary_tree_id.unwrap_or(0) {
            return false;
        }

        if sec != page.secondary_tree_id.unwrap_or(0) {
            return false;
        }

        let have: Vec<i32> = cur
            .get("selectedPerkIds")
            .and_then(|arr| arr.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|n| n.as_i64().map(|x| x as i32))
                    .collect()
            })
            .unwrap_or_default();

        let want = page.collect_perk_ids();

        Self::same_perks(&want, &have)
    }

    async fn apply_page(&self, page: RunePage, api: &dyn LcuApiPort, role: &str) {
        let prim = page.primary_tree_id.unwrap_or(0);
        let sec = page.secondary_tree_id.unwrap_or(0);
        let perks = page.collect_perk_ids();

        if self.same_page(&page, api).await {
            log::info!("RunePicker: identical page already active — skipping");
            return;
        }

        // 1️⃣ Fetch current page ID (same as Python)
        let current = self.fetch_current(api).await;
        let current_id = current
            .as_ref()
            .and_then(|v| v.get("id"))
            .and_then(|v| v.as_i64());

        // 2️⃣ Delete current page just like Python
        if let Some(id) = current_id {
            let path = format!("/lol-perks/v1/pages/{id}");
            match api.delete(&path).await {
                Ok(_) => log::info!("RunePicker: deleted current rune page (id={})", id),
                Err(e) => log::warn!(
                    "RunePicker: could not delete current rune page {}: {}",
                    id,
                    e
                ),
            }
        }

        let body = json!({
            "name": page.name,
            "primaryStyleId": prim,
            "subStyleId": sec,
            "selectedPerkIds": perks,
            "current": true
        });

        match api.post("/lol-perks/v1/pages", Some(&body)).await {
            Ok(_) => log::info!("RunePicker: applied page '{}' ({})", page.name, role),
            Err(e) => log::error!("RunePicker: failed to create new page: {}", e),
        }
    }

    pub async fn apply_for_champion(
        &self,
        name: &str,
        _champ_id: i32,
        role: &str,
        api: &dyn LcuApiPort,
    ) {
        let page = match self.find_best_page(name, Some(role)) {
            Some(p) => p,
            None => {
                log::warn!("RunePicker: no rune page found for {name} ({role})");
                return;
            }
        };

        self.apply_page(page, api, role).await;
    }
}
