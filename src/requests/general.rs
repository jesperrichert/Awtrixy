use crate::SettingsDto;
use crate::client::Warptrixy;
use reqwest::{Error, StatusCode};

impl Warptrixy {
    pub async fn get_settings(&self) -> Result<StatusCode, Error> {
        self.client
            .get(format!("{}/api/settings", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn change_settings(&self, dto: SettingsDto) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/settings", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&dto)
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn do_update(&self) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/doupdate", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn do_reboot(&self) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/reboot", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn erase(&self) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/erase", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn reset_settings(&self) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/resetSettings", &self.config.url))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }
}
