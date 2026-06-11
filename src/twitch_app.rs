use crate::disbot_app::DisBotStatsAPIResponse;
use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::string::ToString;
use warptrixy::{CustomAppDto, TextColorValue, TextValue, WarptrixyClient};

pub async fn twitch_app(client: &WarptrixyClient) {
    let twitch_client_id = std::env::var("TWITCH_CLIENT_ID").unwrap();
    let twitch_client_secret = std::env::var("TWITCH_CLIENT_SECRET").unwrap();

    let client_oauth_token = reqwest::Client::new()
        .post("https://id.twitch.tv/oauth2/token".to_string())
        .form(&TwitchApplicationConfig {
            client_id: twitch_client_id.to_string(),
            client_secret: twitch_client_secret.to_string(),
            grant_type: "client_credentials".to_string(),
        })
        .send()
        .await
        .unwrap()
        .json::<TwitchApplicationOAuthResponse>()
        .await
        .unwrap();

    let twitch_stats = reqwest::Client::new()
        .get("https://api.twitch.tv/helix/channels/followers?broadcaster_id=744716265")
        .header(
            "Authorization",
            format!("Bearer {}", client_oauth_token.access_token),
        )
        .header("Client-Id", twitch_client_id.to_string())
        .send()
        .await
        .unwrap()
        .json::<TwitchUserInfoAPIResponse>()
        .await
        .unwrap();

    let text = " Follower on Twitch".to_string();
    client
        .set_custom_app(
            "twitch_jespersentv".to_string(),
            CustomAppDto {
                text: Option::from(TextValue::Colored(Vec::from([
                    TextColorValue {
                        text: twitch_stats.follower_count.to_string(),
                        color: "#6441a5".to_string(),
                    },
                    TextColorValue {
                        text,
                        color: "#7289da".to_string(),
                    },
                ]))),
                text_case: None,
                top_text: Option::from(true),
                text_offset: None,
                center: Option::from(true),
                color: None,
                gradient: None,
                blink_text: None,
                fade_text: None,
                background: None,
                rainbow: None,
                icon: None,
                push_icon: None,
                repeat: None,
                duration: None,
                hold: None,
                sound: None,
                rtttl: None,
                loop_sound: None,
                bar: None,
                line: None,
                autoscale: None,
                bar_bc: None,
                progress: None,
                progress_c: None,
                progress_bc: None,
                pos: None,
                draw: None,
                lifetime: None,
                lifetime_mode: None,
                stack: None,
                wakeup: None,
                no_scroll: None,
                clients: None,
                scroll_speed: Some(30),
                effect: None,
                effect_settings: None,
                save: None,
                overlay: None,
            },
        )
        .await
        .unwrap();
}

#[derive(Deserialize, Serialize)]
pub struct TwitchUserInfoAPIResponse {
    #[serde(rename = "total")]
    pub follower_count: i32,
}

#[derive(Deserialize, Serialize)]
pub struct TwitchApplicationConfig {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct TwitchApplicationOAuthResponse {
    pub access_token: String,
}
