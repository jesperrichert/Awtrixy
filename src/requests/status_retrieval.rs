use crate::client::Warptrixy;
use reqwest::{Error, StatusCode};

impl Warptrixy {
    pub async fn list_apps(&self) -> Result<StatusCode, Error> {
        self.client
            .get(format!("{}/api/loop", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn list_transitions(&self) -> Result<StatusCode, Error> {
        self.client
            .get(format!("{}/api/transitions", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn list_effects(&self) -> Result<StatusCode, Error> {
        self.client
            .get(format!("{}/api/effects", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn get_stats(&self) -> Result<StatusCode, Error> {
        self.client
            .get(format!("{}/api/stats", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }
}
