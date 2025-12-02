use crate::domain::champ_select::models::ChampSelectSessionPayload;

#[derive(Debug, Clone)]
pub struct BanPickRequest {
    pub role: String,
    pub banned: Vec<i32>,
    pub my_team: Vec<i32>,
    pub their_team: Vec<i32>,
    pub intended_picks: Vec<i32>,
    pub queue_id: i64,
    pub my_pick: Option<i32>,
}

impl BanPickRequest {
    pub fn from_session(session: &ChampSelectSessionPayload, role: &str) -> Self {
        let mut banned = Vec::new();
        let mut intended = Vec::new();
        let mut my_team_locked = Vec::new();
        let mut their_team_locked = Vec::new();
        let mut my_pick = None;

        let my_team_ids: std::collections::HashSet<i64> =
            session.myTeam.iter().map(|p| p.cellId).collect();
        let enemy_team_ids: std::collections::HashSet<i64> =
            session.theirTeam.iter().map(|p| p.cellId).collect();

        for phase in &session.actions {
            for action in phase {
                if action.championId == 0 {
                    continue;
                }

                match action.action_type.as_str() {
                    "ban" => {
                        if action.completed {
                            banned.push(action.championId as i32);
                        }
                    }

                    "pick" => {
                        if action.actorCellId == session.localPlayerCellId {
                            my_pick = Some(action.championId as i32);
                        }

                        if action.completed {
                            if my_team_ids.contains(&action.actorCellId) {
                                my_team_locked.push(action.championId as i32);
                            } else if enemy_team_ids.contains(&action.actorCellId) {
                                their_team_locked.push(action.championId as i32);
                            }
                        } else {
                            intended.push(action.championId as i32);
                        }
                    }

                    _ => {}
                }
            }
        }

        Self {
            role: role.to_string(),
            banned,
            my_team: my_team_locked,
            their_team: their_team_locked,
            intended_picks: intended,
            queue_id: session.queueId,
            my_pick,
        }
    }
}
