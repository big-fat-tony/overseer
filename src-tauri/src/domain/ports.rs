use std::sync::Arc;
use serde_json::Value;
use async_trait::async_trait;
use crate::application::features::auto_pick_ban::ban_pick_request::BanPickRequest;
use crate::domain::champ_select::decision::Decision;
use crate::domain::champ_select::models::{ChampSelectSessionPayload, CsAction};
use crate::domain::events::LeagueEvent;
use crate::domain::log_entry::LogEntry;
use anyhow::Result;

pub trait LeagueEventPublisherPort: Send + Sync {
    fn subscribe(&self, subscriber: Arc<dyn LeagueEventSubscriber>);
    fn unsubscribe(&self, target: &Arc<dyn LeagueEventSubscriber>);
    fn publish(&self, event: &LeagueEvent);
}

pub trait LogPublisherPort: Send + Sync {
    fn publish(&self, entry: LogEntry);
}

pub trait LeagueEventSubscriber: Send + Sync {
    fn on_event(&self, event: &LeagueEvent);
}

pub trait IngameEventPublisherPort: Send + Sync {
    fn subscribe(&self, subscriber: Arc<dyn IngameEventSubscriber>) -> u64;
    fn unsubscribe(&self, id: u64);
    fn publish(&self, event: &Value);
}

#[async_trait]
pub trait IngameEventSubscriber: Send + Sync {
    async fn on_ingame_event(&self, event: &Value);
}

pub trait LockfilePort: Send + Sync {
    fn read_lockfile(&self) -> anyhow::Result<LockfileData>;
}

#[derive(Debug, Clone)]
pub struct LockfileData {
    pub port: u16,
    pub password: String,
}

#[async_trait]
pub trait LcuApiPort: Send + Sync {
    async fn get(&self, path: &str) -> anyhow::Result<Value>;
    async fn post(&self, path: &str, body: Option<&Value>) -> anyhow::Result<Option<Value>>;
    async fn put(&self, path: &str, body: Option<&Value>) -> anyhow::Result<Option<Value>>;
    async fn patch(&self, path: &str, body: Option<&Value>) -> anyhow::Result<Option<Value>>;
    async fn delete(&self, path: &str) -> anyhow::Result<Option<Value>>;
}

#[async_trait::async_trait]
pub trait HovererPort: Send + Sync {
    async fn hover(&self, session: &ChampSelectSessionPayload, role: &str);
}

#[async_trait::async_trait]
pub trait PickerPort: Send + Sync {
    async fn pick(&self, session: &ChampSelectSessionPayload, action: &CsAction, role: &str);
}

#[async_trait::async_trait]
pub trait BannerPort: Send + Sync {
    async fn ban(&self, session: &ChampSelectSessionPayload, action: &CsAction, role: &str);
}

#[async_trait]
pub trait PickingStrategy: Send + Sync {
    async fn decide(&self, req: &BanPickRequest) -> Decision;
}

#[async_trait]
pub trait BanStrategy: Send + Sync {
    async fn decide(&self, req: &BanPickRequest) -> Decision;
}

#[async_trait]
pub trait ChampionResolverPort: Send + Sync {
    async fn refresh_cache(&self);
    async fn resolve_id(&self, name: &str) -> Option<i32>;
    async fn resolve_name(&self, champ_id: i32) -> Option<String>;
}

#[async_trait]
pub trait DataDragonApiPort: Send + Sync {
    async fn get_versions(&self) -> Result<Vec<String>>;
    async fn get_champions_json(&self, version: &str) -> Result<Value>;
}