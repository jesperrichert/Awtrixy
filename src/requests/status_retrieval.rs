use crate::client::Warptrixy;
use crate::{Config, CustomAppDto};
use reqwest::{Error, Response};

impl Warptrixy {
    pub async fn list_apps(&self) -> Result<Response, Error> {
        self.client
            .get(format!("{}/api/loop", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }

    pub async fn list_transitions(&self) -> Result<Response, Error> {
        self.client
            .get(format!("{}/api/transitions", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }

    pub async fn list_effects(&self) -> Result<Response, Error> {
        self.client
            .get(format!("{}/api/effects", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }

    pub async fn get_stats(&self) -> Result<Response, Error> {
        self.client
            .get(format!("{}/api/stats", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }
}
