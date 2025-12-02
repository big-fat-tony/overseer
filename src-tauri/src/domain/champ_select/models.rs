use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CsAction {
    pub id: i64,
    pub actorCellId: i64,
    pub championId: i64,
    pub completed: bool,
    pub isInProgress: bool,
    #[serde(rename = "type")]
    pub action_type: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CsTimer {
    pub phase: String,
    pub adjustedTimeLeftInPhase: Option<i64>,
    pub totalTimeInPhase: Option<i64>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CsPlayer {
    pub cellId: i64,
    pub championId: i64,
    pub assignedPosition: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct ChampSelectSessionPayload {
    pub actions: Vec<Vec<CsAction>>,
    pub myTeam: Vec<CsPlayer>,
    pub theirTeam: Vec<CsPlayer>,

    pub localPlayerCellId: i64,
    pub queueId: i64,

    pub timer: CsTimer,
}
