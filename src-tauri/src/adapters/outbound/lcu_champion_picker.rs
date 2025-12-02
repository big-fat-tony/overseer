use std::sync::Arc;
use log::info;

use crate::domain::ports::{LcuApiPort, PickerPort, PickingStrategy};
use crate::domain::champ_select::models::{ChampSelectSessionPayload, CsAction};
use crate::application::features::auto_pick_ban::ban_pick_request::BanPickRequest;

pub struct ChampionPicker<S> {
    api: Arc<dyn LcuApiPort>,
    strategy: Arc<S>,
}

impl<S> ChampionPicker<S> {
    pub fn new(api: Arc<dyn LcuApiPort>, strategy: Arc<S>) -> Self {
        Self { api, strategy }
    }
}

#[async_trait::async_trait]
impl<S> PickerPort for ChampionPicker<S>
where
    S: PickingStrategy + 'static,
{
    async fn pick(
        &self,
        session: &ChampSelectSessionPayload,
        action: &CsAction,
        role: &str,
    ) {
        let req = BanPickRequest::from_session(session, role);
        info!("{:?}", req);

        let decision = self.strategy.decide(&req).await;
        if !decision.has_choice() {
            info!("No decision for {:?}", req);
            return;
        }

        if !action.isInProgress {
            info!("Pick action not in progress, skipping");
            return;
        }

        let cid = decision.champion_id() as i64;

        info!("[Pick] Locking champion {} for role={}", cid, role);

        let payload = serde_json::json!({
            "championId": cid,
            "completed": true
        });

        let _ = self.api.patch(
            &format!("/lol-champ-select/v1/session/actions/{}", action.id),
            Some(&payload),
        ).await;
    }
}
