use crate::client::Warptrixy;
use reqwest::{Error, Response};
use serde_json::json;

impl Warptrixy {
    pub async fn set_power(&self, online: bool) -> Result<Response, Error> {
        self.client
            .post(format!("{}/api/loop", &self.config.url))
            .json(&json!({
                "power": online,
            }))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }

    pub async fn set_sleep(&self, time: i32) -> Result<Response, Error> {
        self.client
            .post(format!("{}/api/loop", &self.config.url))
            .json(&json!({
                "sleep": time
            }))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }
}
