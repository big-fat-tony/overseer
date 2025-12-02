use async_trait::async_trait;
use serde_json::Value;
use crate::domain::ports::LcuApiPort;

pub struct NullLcuApi;

#[async_trait]
impl LcuApiPort for NullLcuApi {
    async fn get(&self, _: &str) -> anyhow::Result<Value> {
        Err(anyhow::anyhow!("LCU unavailable"))
    }

    async fn post(&self, _: &str, _: Option<&Value>) -> anyhow::Result<Option<Value>> {
        Err(anyhow::anyhow!("LCU unavailable"))
    }

    async fn put(&self, _: &str, _: Option<&Value>) -> anyhow::Result<Option<Value>> {
        Err(anyhow::anyhow!("LCU unavailable"))
    }

    async fn patch(&self, _: &str, _: Option<&Value>) -> anyhow::Result<Option<Value>> {
        log::info!("Was going to patch, but lcu is not available");
        Err(anyhow::anyhow!("LCU unavailable"))
    }

    async fn delete(&self, _: &str) -> anyhow::Result<Option<Value>> {
        Err(anyhow::anyhow!("LCU unavailable"))
    }
}
