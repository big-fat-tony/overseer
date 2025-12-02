use std::sync::Arc;
use log::info;

use crate::domain::ports::LcuApiPort;
use crate::domain::champ_select::models::{ChampSelectSessionPayload, CsAction};
use crate::application::features::auto_pick_ban::ban_pick_request::BanPickRequest;
use crate::domain::ports::{PickingStrategy, HovererPort};

pub struct ChampionHoverer<S> {
    api: Arc<dyn LcuApiPort>,
    strategy: Arc<S>,
}

impl<S> ChampionHoverer<S> {
    pub fn new(api: Arc<dyn LcuApiPort>, strategy: Arc<S>) -> Self {
        Self { api, strategy }
    }
}

#[async_trait::async_trait]
impl<S> HovererPort for ChampionHoverer<S>
where
    S: PickingStrategy + 'static,
{
    async fn hover(&self, session: &ChampSelectSessionPayload, role: &str) {
        let req = BanPickRequest::from_session(session, role);
        info!("{:?}", req);

        let decision = self.strategy.decide(&req).await;
        if !decision.has_choice() {
            info!("No decision for {:?}", req);
            return;
        }

        let Some(action) = find_pick_action(session) else {
            info!("No action found");
            return;
        };

        if action.completed {
            info!("Action already completed, skipping");
            return;
        }

        // FIXED: convert i32 → i64
        let cid = decision.champion_id() as i64;

        if action.championId == cid {
            info!("Preferred champion already set, skipping");
            return;
        }

        info!("[Hover] Champion {} for role={}", cid, role);

        // FIXED: &Value instead of Value
        let payload = serde_json::json!({
            "championId": cid
        });

        let _ = self.api.patch(
            &format!("/lol-champ-select/v1/session/actions/{}", action.id),
            Some(&payload),
        ).await;
    }
}

fn find_pick_action(session: &ChampSelectSessionPayload) -> Option<&CsAction> {
    for row in &session.actions {
        for action in row {
            if action.action_type == "pick"
                && action.actorCellId == session.localPlayerCellId
            {
                return Some(action);
            }
        }
    }
    None
}
