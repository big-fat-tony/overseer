use std::sync::Arc;
use log::info;

use crate::domain::ports::{LcuApiPort, BannerPort, BanStrategy};
use crate::domain::champ_select::models::{ChampSelectSessionPayload, CsAction};
use crate::application::features::auto_pick_ban::ban_pick_request::BanPickRequest;

pub struct ChampionBanner<S> {
    api: Arc<dyn LcuApiPort>,
    strategy: Arc<S>,
}

impl<S> ChampionBanner<S> {
    pub fn new(api: Arc<dyn LcuApiPort>, strategy: Arc<S>) -> Self {
        Self { api, strategy }
    }
}

#[async_trait::async_trait]
impl<S> BannerPort for ChampionBanner<S>
where
    S: BanStrategy + 'static,
{
    async fn ban(
        &self,
        session: &ChampSelectSessionPayload,
        action: &CsAction,
        role: &str,
    ) {
        let req = BanPickRequest::from_session(session, role);
        info!("{:?}", req);

        let decision = self.strategy.decide(&req).await;
        if !decision.has_choice() {
            info!("No ban decision for {:?}", req);
            return;
        }

        // No need to check isInProgress for bans (matches Python behavior)

        let cid = decision.champion_id() as i64;

        info!("[Ban] Champion {} for role={}", cid, role);

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
