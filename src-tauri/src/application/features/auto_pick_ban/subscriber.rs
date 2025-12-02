use log::info;
use std::sync::Arc;
use tokio::task;

use crate::domain::events::{EventType, LeagueEvent};
use crate::domain::ports::{BannerPort, HovererPort, LeagueEventSubscriber, PickerPort};

use crate::domain::champ_select::models::{ChampSelectSessionPayload, CsAction};

pub struct AutoPickBanSubscriber {
    hoverer: Arc<dyn HovererPort>,
    picker: Arc<dyn PickerPort>,
    banner: Arc<dyn BannerPort>,
}

impl AutoPickBanSubscriber {
    pub fn new(
        hoverer: Arc<dyn HovererPort>,
        picker: Arc<dyn PickerPort>,
        banner: Arc<dyn BannerPort>,
    ) -> Arc<Self> {
        Arc::new(Self {
            hoverer,
            picker,
            banner,
        })
    }
}

impl LeagueEventSubscriber for AutoPickBanSubscriber {
    fn on_event(&self, event: &LeagueEvent) {
        if event.event_type != EventType::ChampSelectSession {
            return;
        }

        let source = event.data.get("payload").unwrap_or(&event.data);

        let Ok(session) = serde_json::from_value::<ChampSelectSessionPayload>(source.clone())
        else {
            info!("[APB] Failed to parse ChampSelectSession");
            return;
        };
        let hoverer = Arc::clone(&self.hoverer);
        let picker = Arc::clone(&self.picker);
        let banner = Arc::clone(&self.banner);

        task::spawn(async move {
            let phase = session.timer.phase.as_str();
            info!("[APB] Phase = {}", phase);

            match phase {
                "PLANNING" => {
                    info!("[APB] In PLANNING phase");

                    if let Some(role) = resolve_role(&session) {
                        info!("[APB] Role = {}", role);
                        info!("[APB] Calling hoverer.hover()");
                        let _ = hoverer.hover(&session, &role).await;
                    } else {
                        info!("[APB] No role resolved");
                    }
                }

                "BAN_PICK" => {
                    info!("[APB] In BAN_PICK phase");

                    if let Some(action) = find_active_action(&session) {
                        info!("[APB] Active action type = {}", action.action_type);

                        if let Some(role) = resolve_role(&session) {
                            info!("[APB] Role = {}", role);

                            match action.action_type.as_str() {
                                "pick" => {
                                    info!("[APB] Calling picker.pick()");
                                    let _ = picker.pick(&session, action, &role).await;
                                }
                                "ban" => {
                                    info!("[APB] Calling banner.ban()");
                                    let _ = banner.ban(&session, action, &role).await;
                                }
                                other => {
                                    info!("[APB] Unknown action type {}", other);
                                }
                            }
                        } else {
                            info!("[APB] No role resolved");
                        }
                    } else {
                        info!("[APB] No active action found");
                    }
                }

                other => {
                    info!("[APB] Ignoring phase {}", other);
                }
            }
        });
    }
}

fn find_active_action(s: &ChampSelectSessionPayload) -> Option<&CsAction> {
    for grp in &s.actions {
        if grp.iter().any(|a| a.isInProgress) {
            for a in grp {
                if a.actorCellId == s.localPlayerCellId && !a.completed {
                    return Some(a);
                }
            }
        }
    }
    None
}

fn resolve_role(s: &ChampSelectSessionPayload) -> Option<String> {
    for p in &s.myTeam {
        if p.cellId == s.localPlayerCellId {
            let role = p
                .assignedPosition
                .clone()
                .unwrap_or_default()
                .to_lowercase();

            if role.is_empty() {
                if s.queueId == 3140 {
                    return Some("support".into());
                } else {
                    return None;
                }
            }

            return Some(role);
        }
    }

    None
}
