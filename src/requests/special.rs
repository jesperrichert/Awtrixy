use crate::{ColoredIndicatorDto, Config, CustomAppDto, IndicatorType, MoodLightingDto};
use reqwest::{Error, Response};
use serde::Serialize;
use serde_json::json;
use crate::client::Warptrixy;

impl Warptrixy {
    pub async fn play_sound(&self,sound: String) -> Result<Response, Error> {
        self.client
            .post(format!("{}/api/sound", &self.config.url))
            .json(&json!({
            "sound": sound,
        }))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }

    pub async fn play_moodlight(&self,dto: MoodLightingDto) -> Result<Response, Error> {
        self.client
            .post(format!("{}/api/moodlight", &self.config.url))
            .json(&dto)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }

    pub async fn set_colored_indicators(
        &self,
        indicator: IndicatorType,
        dto: ColoredIndicatorDto,
    ) -> Result<Response, Error> {
        self.client
            .post(format!("{}/api/indicator{}", &self.config.url, indicator.to_int()))
            .json(&dto)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
    }
}
