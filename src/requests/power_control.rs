use crate::client::Warptrixy;
use reqwest::{Error, Response, StatusCode};
use serde_json::json;

impl Warptrixy {
    pub async fn set_power(&self, online: bool) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/power", &self.config.url))
            .json(&json!({
                "power": online,
            }))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn set_sleep(&self, time: i32) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/sleep", &self.config.url))
            .json(&json!({
                "sleep": time
            }))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }
}
