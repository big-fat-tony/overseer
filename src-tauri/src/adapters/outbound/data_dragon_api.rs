use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

use crate::domain::ports::DataDragonApiPort;

pub struct DataDragonApiAdapter {
    client: Client,
}

impl DataDragonApiAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl DataDragonApiPort for DataDragonApiAdapter {
    async fn get_versions(&self) -> Result<Vec<String>> {
        let url = "https://ddragon.leagueoflegends.com/api/versions.json";
        let resp = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Vec<String>>()
            .await?;
        Ok(resp)
    }

    async fn get_champions_json(&self, version: &str) -> Result<Value> {
        let url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/en_US/champion.json",
            version
        );

        let resp = self.client.get(&url).send().await?;
        resp.error_for_status_ref()?;
        Ok(resp.json::<Value>().await?)
    }
}
