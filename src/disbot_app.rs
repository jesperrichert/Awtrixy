use serde::{Deserialize, Serialize};
use std::string::ToString;
use warptrixy::{ColorValue, CustomAppDto, TextColorValue, TextValue, WarptrixyClient};

pub async fn disbot_app(client: &WarptrixyClient) {
    let disbot_stats = reqwest::get("https://api.disbot.app/v2/bot/stats")
        .await
        .unwrap()
        .json::<DisBotStatsAPIResponse>()
        .await
        .unwrap();
    let text = " DisBot Guilds".to_string();
    client
        .set_custom_app(
            "twitch_jespersentv".to_string(),
            CustomAppDto {
                text: Option::from(TextValue::Colored(Vec::from([
                    TextColorValue {
                        text: disbot_stats.guild_count.to_string(),
                        color: "#FFA500".to_string(),
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
                icon: Option::from(DISBOT_LOGO_BASE64.to_string()),
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
pub struct DisBotStatsAPIResponse {
    #[serde(rename = "guildcound")]
    pub guild_count: i32,
}

const DISBOT_LOGO_BASE64: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAABP0lEQVR4AdxU0RWCMAwEJ5FNdBJ1EmAScRLcBDapd7Hl0ZRS+oQfeTnb5pIcpK2n4uDnzwWMMb0JnyGnq6kWvReKnRd8UdeqQFmWbTl7olVWiFWBlbzNVCCAltfAAATmqgbE18Gcu4txoyeAOAY0ILP6jHgac562BtcCTwCeG0C7zlq/aYqkCqDV/HHQAhcSqLh0ekhFgZzRkvwSOy0KLTARe02SAugpL1uvBWN+HScCNtho0q7ZNsIup4E+YnK4CerR5KVEAER2z5GTMtkTEcAGyY2NZPDYdgtczF+gHu3BHBHgJAZEUlyC5zEx/zyG86QAg36BFpC9wA7xRmfVRY7bcKnhkrXAyxK88sjZbsiTU4PR1cA0vGhUZ785SkDGD09Ng73xDoT3BSBHoAOy/4uQUwGtfiFPQJN7rA8X+AAAAP//Dg2DOQAAAAZJREFUAwBsKMIxWd6BewAAAABJRU5ErkJggg==";
