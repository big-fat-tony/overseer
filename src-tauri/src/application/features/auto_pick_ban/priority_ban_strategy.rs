use std::sync::Arc;
use async_trait::async_trait;
use log::{info, warn};
use crate::application::features::auto_pick_ban::ban_pick_request::BanPickRequest;
use crate::domain::champ_select::ban_preferences::BanPreferences;
use crate::domain::champ_select::decision::{Decision, choose, no_decision};
use crate::domain::ports::BanStrategy;

pub struct PriorityBanStrategy {
    prefs: Arc<BanPreferences>,
}

impl PriorityBanStrategy {
    pub fn new(prefs: Arc<BanPreferences>) -> Self {
        Self { prefs }
    }
}

#[async_trait]
impl BanStrategy for PriorityBanStrategy {
    async fn decide(&self, request: &BanPickRequest) -> Decision {
        info!("[PriorityBanStrategy] Deciding ban from {:?}", request);

        let role_prefs = self.prefs.get_for_role(&request.role).await;

        info!(
            "[PriorityBanStrategy] Role={}, Prefs={:?}",
            request.role, role_prefs
        );
        info!(
            "[PriorityBanStrategy] Banned={:?}, MyTeam={:?}",
            request.banned, request.my_team
        );

        for cid in role_prefs {
            info!("[PriorityBanStrategy] Checking champion_id={cid}");

            if request.banned.contains(&cid) {
                info!("[PriorityBanStrategy] Skipping {cid} (already banned)");
                continue;
            }

            if request.intended_picks.contains(&cid) {
                info!("[PriorityBanStrategy] Skipping {cid} (ally using)");
                continue;
            }

            info!("[PriorityBanStrategy] ✔ Decided to ban champion_id={cid}");
            return choose(cid);
        }

        warn!(
            "[PriorityBanStrategy] ⚠ No valid ban target found for role={}",
            request.role
        );

        no_decision()
    }
}
