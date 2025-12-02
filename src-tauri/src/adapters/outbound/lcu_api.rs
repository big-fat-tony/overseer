use std::sync::Arc;
use async_trait::async_trait;
use reqwest::{Client, Response};
use serde_json::Value;
use crate::domain::ports::{LcuApiPort, LockfilePort};

pub struct LcuApiAdapter {
    client: Client,
    lockfile: Arc<dyn LockfilePort>,
}

impl LcuApiAdapter {
    pub fn new(lockfile: Arc<dyn LockfilePort>) -> anyhow::Result<Self> {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        Ok(Self { client, lockfile })
    }

    fn prepare(&self) -> anyhow::Result<(String, (String, String))> {
        let lf = self.lockfile.read_lockfile()?;
        let url = format!("https://127.0.0.1:{}", lf.port);
        let auth = ("riot".into(), lf.password);
        Ok((url, auth))
    }

    fn url(base: &str, path: &str) -> String {
        format!("{}{}", base, path)
    }

    fn auth(rb: reqwest::RequestBuilder, auth: &(String, String)) -> reqwest::RequestBuilder {
        rb.basic_auth(&auth.0, Some(&auth.1))
    }

    async fn json_or_none(resp: Response) -> anyhow::Result<Option<Value>> {
        if resp.status() == 204 {
            return Ok(None);
        }
        let text = resp.text().await?;
        if text.is_empty() {
            return Ok(None);
        }
        Ok(Some(serde_json::from_str(&text)?))
    }
}

#[async_trait]
impl LcuApiPort for LcuApiAdapter {
    async fn get(&self, path: &str) -> anyhow::Result<Value> {
        let (base, auth) = self.prepare()?;
        let resp = Self::auth(self.client.get(Self::url(&base, path)), &auth)
            .send()
            .await?;
        resp.error_for_status_ref()?;
        Ok(resp.json().await?)
    }

    async fn post(&self, path: &str, body: Option<&Value>) -> anyhow::Result<Option<Value>> {
        let (base, auth) = self.prepare()?;
        let req = Self::auth(self.client.post(Self::url(&base, path)), &auth);
        let req = if let Some(b) = body { req.json(b) } else { req };
        let resp = req.send().await?;
        resp.error_for_status_ref()?;
        Self::json_or_none(resp).await
    }

    async fn put(&self, path: &str, body: Option<&Value>) -> anyhow::Result<Option<Value>> {
        let (base, auth) = self.prepare()?;
        let req = Self::auth(self.client.put(Self::url(&base, path)), &auth);
        let req = if let Some(b) = body { req.json(b) } else { req };
        let resp = req.send().await?;
        resp.error_for_status_ref()?;
        Self::json_or_none(resp).await
    }

    async fn patch(&self, path: &str, body: Option<&Value>) -> anyhow::Result<Option<Value>> {
        let (base, auth) = self.prepare()?;
        let req = Self::auth(self.client.patch(Self::url(&base, path)), &auth);
        let req = if let Some(b) = body { req.json(b) } else { req };
        let resp = req.send().await?;
        resp.error_for_status_ref()?;
        Self::json_or_none(resp).await
    }

    async fn delete(&self, path: &str) -> anyhow::Result<Option<Value>> {
        let (base, auth) = self.prepare()?;
        let resp = Self::auth(self.client.delete(Self::url(&base, path)), &auth)
            .send()
            .await?;
        if resp.status() == 204 {
            return Ok(None);
        }
        resp.error_for_status_ref()?;
        Self::json_or_none(resp).await
    }
}
