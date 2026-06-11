use crate::client::Warptrixy;
use crate::{Config, CustomAppDto};
use reqwest::{Error, StatusCode};

impl Warptrixy {
    pub async fn send_notification(&self, dto: CustomAppDto) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/notify", &self.config.url))
            .json(&dto)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn dismiss_notification(&self) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/notify/dismiss", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }
}
