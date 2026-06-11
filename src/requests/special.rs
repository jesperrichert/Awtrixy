use crate::client::Warptrixy;
use crate::{ColoredIndicatorDto, IndicatorType, MoodLightingDto};
use reqwest::{Error, StatusCode};
use serde_json::json;

impl Warptrixy {
    pub async fn play_sound(&self, sound: String) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/sound", &self.config.url))
            .json(&json!({
                "sound": sound,
            }))
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn play_moodlight(&self, dto: MoodLightingDto) -> Result<StatusCode, Error> {
        self.client
            .post(format!("{}/api/moodlight", &self.config.url))
            .json(&dto)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }

    pub async fn set_colored_indicators(
        &self,
        indicator: IndicatorType,
        dto: ColoredIndicatorDto,
    ) -> Result<StatusCode, Error> {
        self.client
            .post(format!(
                "{}/api/indicator{}",
                &self.config.url,
                indicator.to_int()
            ))
            .json(&dto)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await
            .and_then(|resp| Ok(resp.status()))
    }
}
