use crate::application::features::rune_picker::request::RunePickRequest;
use crate::domain::ports::LcuApiPort;
use crate::domain::rune_page_manager::RunePageManager;
use std::sync::Arc;

pub struct RunePickerService {
    api: Arc<dyn LcuApiPort>,
    pages: Arc<RunePageManager>,
}

impl RunePickerService {
    pub fn new(api: Arc<dyn LcuApiPort>, pages: Arc<RunePageManager>) -> Self {
        Self { api, pages }
    }

    pub async fn apply(&self, req: RunePickRequest) {
        let name = req.champion_name;
        let champ_id = req.champion_id;
        let role = req.role;

        log::info!(
            "RunePickerService: Applying runes for {} ({}) [{}]",
            name,
            role,
            champ_id
        );

        self.pages
            .apply_for_champion(&name, champ_id, &role, self.api.as_ref())
            .await;
    }
}
