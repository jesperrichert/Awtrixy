use crate::client::Warptrixy;
use crate::{AppSwitchMode, CustomAppDto};
use reqwest::{Error, StatusCode};
use serde_json::json;

impl Warptrixy {
    pub async fn set_custom_app(
        &self,
        app_name: String,
        dto: CustomAppDto,
    ) -> Result<StatusCode, Error> {
        self.client
            .post(format!(
                "{}/api/custom?name={}",
                &self.config.url, &app_name
            ))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&dto)
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn delete_custom_app(&self, app_name: String) -> Result<StatusCode, Error> {
        self.client
            .post(format!(
                "{}/api/custom?name={}",
                &self.config.url, &app_name
            ))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn switch_app(&self, mode: AppSwitchMode) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/{}", &self.config.url, mode.to_route()))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn switch_specific_app(&self, name: String) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/switch", &self.config.url))
            .json(&json!({ "name": name }))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }
}
