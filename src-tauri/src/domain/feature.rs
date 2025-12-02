use std::{fmt, str::FromStr};

#[derive(serde::Serialize)]
pub struct FeatureInfo {
    pub id: String,
    pub name: String,
    pub enabled: bool,
}

/// A runtime-togglable unit of functionality.
/// Implementations handle their own subscriptions in `enable` and undo them in `disable`.
pub trait Feature: Send + Sync {
    fn id(&self) -> FeatureId;
    fn display_name(&self) -> String;
    fn enable(&self);
    fn disable(&self);

    fn configure(&self, _settings: serde_json::Value) {
        // default: do nothing
    }
}

/// Every distinct feature your app can provide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatureId {
    IngameLog,
    LeagueLog,
    MatchReady,
    AutoPickBan,
    RunePicker,
}

impl fmt::Display for FeatureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeatureId::IngameLog => write!(f, "IngameLog"),
            FeatureId::LeagueLog => write!(f, "LeagueLog"),
            FeatureId::MatchReady => write!(f, "MatchReady"),
            FeatureId::AutoPickBan => write!(f, "AutoPickBan"),
            FeatureId::RunePicker => write!(f, "RunePicker"),
        }
    }
}

impl FromStr for FeatureId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "IngameLog" => Ok(FeatureId::IngameLog),
            "LeagueLog" => Ok(FeatureId::LeagueLog),
            "MatchReady" => Ok(FeatureId::MatchReady),
            "AutoPickBan" => Ok(FeatureId::AutoPickBan),
            "RunePicker" => Ok(FeatureId::RunePicker),
            _ => Err(anyhow::anyhow!("Unknown feature id: {}", s)),
        }
    }
}
