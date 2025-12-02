use std::sync::Arc;
use log::{info, warn};
use crate::application::features::auto_pick_ban::ban_pick_request::BanPickRequest;
use crate::application::features::auto_pick_ban::champion_preferences::ChampionPreferences;
use crate::domain::champ_select::decision::{Decision, no_decision, choose};
use crate::domain::ports::PickingStrategy;

pub struct PriorityPickStrategy {
    prefs: Arc<ChampionPreferences>,
}

impl PriorityPickStrategy {
    pub fn new(prefs: Arc<ChampionPreferences>) -> Self {
        Self { prefs }
    }
}

#[async_trait::async_trait]
impl PickingStrategy for PriorityPickStrategy {
    async fn decide(&self, request: &BanPickRequest) -> Decision {
        info!("[PriorityPickStrategy] Picking from request={:?}", request);

        let role_prefs = self.prefs.get_for_role(&request.role).await;
        info!("[PriorityPickStrategy] Role={}, Prefs={:?}", request.role, role_prefs);
        info!(
            "[PriorityPickStrategy] Banned={:?}, TheirTeam={:?}, MyTeam={:?}",
            request.banned, request.their_team, request.my_team
        );

        for cid in role_prefs {
            // Match Python: if your current pick equals the target, return it
            if let Some(my) = request.my_pick {
                if my == cid {
                    return choose(cid);
                }
            }

            info!("[PriorityPickStrategy] Checking champion_id={cid}");

            if request.banned.contains(&cid) {
                info!("[PriorityPickStrategy] Skipping {cid} (banned)");
                continue;
            }

            if request.their_team.contains(&cid) {
                info!("[PriorityPickStrategy] Skipping {cid} (enemy picked)");
                continue;
            }

            if request.my_team.contains(&cid) {
                info!("[PriorityPickStrategy] Skipping {cid} (ally picked)");
                continue;
            }

            info!("[PriorityPickStrategy] ✔ Decided to pick champion_id={cid}");
            return choose(cid);
        }

        warn!(
            "[PriorityPickStrategy] ⚠ No valid champion found for role={}",
            request.role
        );

        no_decision()
    }
}
