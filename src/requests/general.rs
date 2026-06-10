use crate::client::Warptrixy;
use crate::{Config, SettingsDto};
use reqwest::{Client, Error, Response};

impl Warptrixy {
    pub async fn change_settings(&self, dto: SettingsDto) -> Result<Response, Error> {
        self.client
            .post(format!("{}/api/settings", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&dto)
            .send()
            .await
    }

    pub async fn do_update(&self) -> Result<Response, Error> {
        self.client
            .post(format!("{}/api/doupdate", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }

    pub async fn do_reboot(&self) -> Result<Response, Error> {
        self.client
            .post(format!("{}/api/reboot", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }

    pub async fn erase(&self) -> Result<Response, Error> {
        self.client
            .post(format!("{}/api/erase", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }

    pub async fn reset_settings(&self) -> Result<Response, Error> {
        self.client
            .post(format!("{}/api/resetSettings", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }
}
