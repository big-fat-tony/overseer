use crate::domain::ports::ChampionResolverPort;
use serde_json::Value;

pub struct RunePickRequest {
    pub champion_id: i32,
    pub champion_name: String,
    pub role: String,
}

impl RunePickRequest {
    pub async fn from_session(
        session: &Value,
        resolver: &dyn ChampionResolverPort,
    ) -> Option<Self> {
        let cell = session.get("localPlayerCellId")?.as_i64()? as i32;
        let team = session.get("myTeam")?.as_array()?;

        let player = team.iter().find(|p| {
            p.get("cellId")
                .and_then(|v| v.as_i64())
                .map(|id| id as i32 == cell)
                .unwrap_or(false)
        })?;

        let champ = player.get("championId")?.as_i64()? as i32;
        if champ == 0 {
            return None;
        }

        let queue = session.get("queueId")?.as_i64()? as i32;

        let mut role = player
            .get("assignedPosition")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_lowercase();

        let role = match role.as_str() {
            "utility" => "support".into(),
            "middle" => "mid".into(),
            "" if queue == 3140 => "support".into(),
            "" => "unknown".into(),
            x => x.into(),
        };

        let name = resolver.resolve_name(champ).await?;

        Some(Self {
            champion_id: champ,
            champion_name: name,
            role,
        })
    }
}
