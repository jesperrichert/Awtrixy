use serde::{Deserialize, Serialize};
use crate::{ColorValue};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TransitionEffect {
    Random = 0,
    Slide = 1,
    Dim = 2,
    Zoom = 3,
    Rotate = 4,
    Pixelate = 5,
    Curtain = 6,
    Ripple = 7,
    Blink = 8,
    Reload = 9,
    Fade = 10,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TimeMode {
    Style0 = 0,
    Style1 = 1,
    Style2 = 2,
    Style3 = 3,
    Style4 = 4,
    Style5 = 5,
    Style6 = 6,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OverlayEffect {
    Clear,
    Snow,
    Rain,
    Drizzle,
    Storm,
    Thunder,
    Frost,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SettingsDto {
    #[serde(rename = "ATIME", skip_serializing_if = "Option::is_none")]
    pub app_time: Option<u32>,
    #[serde(rename = "TEFF", skip_serializing_if = "Option::is_none")]
    pub transition_effect: Option<TransitionEffect>,
    #[serde(rename = "TSPEED", skip_serializing_if = "Option::is_none")]
    pub transition_speed: Option<u32>,
    #[serde(rename = "TCOL", skip_serializing_if = "Option::is_none")]
    pub text_color: Option<ColorValue>,
    #[serde(rename = "TMODE", skip_serializing_if = "Option::is_none")]
    pub time_mode: Option<TimeMode>,
    #[serde(rename = "CHCOL", skip_serializing_if = "Option::is_none")]
    pub calendar_header_color: Option<ColorValue>,
    #[serde(rename = "CBCOL", skip_serializing_if = "Option::is_none")]
    pub calendar_body_color: Option<ColorValue>,
    #[serde(rename = "CTCOL", skip_serializing_if = "Option::is_none")]
    pub calendar_text_color: Option<ColorValue>,
    #[serde(rename = "WD", skip_serializing_if = "Option::is_none")]
    pub weekday_display: Option<bool>,
    #[serde(rename = "WDCA", skip_serializing_if = "Option::is_none")]
    pub weekday_color_active: Option<ColorValue>,
    #[serde(rename = "WDCI", skip_serializing_if = "Option::is_none")]
    pub weekday_color_inactive: Option<ColorValue>,
    #[serde(rename = "BRI", skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
    #[serde(rename = "ABRI", skip_serializing_if = "Option::is_none")]
    pub auto_brightness: Option<bool>,
    #[serde(rename = "ATRANS", skip_serializing_if = "Option::is_none")]
    pub auto_transition: Option<bool>,
    #[serde(rename = "CCORRECTION", skip_serializing_if = "Option::is_none")]
    pub color_correction: Option<[u8; 3]>,
    #[serde(rename = "CTEMP", skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<[u8; 3]>,
    #[serde(rename = "TFORMAT", skip_serializing_if = "Option::is_none")]
    pub time_format: Option<String>,
    #[serde(rename = "DFORMAT", skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[serde(rename = "SOM", skip_serializing_if = "Option::is_none")]
    pub start_on_monday: Option<bool>,
    #[serde(rename = "CEL", skip_serializing_if = "Option::is_none")]
    pub celsius: Option<bool>,
    #[serde(rename = "BLOCKN", skip_serializing_if = "Option::is_none")]
    pub block_navigation: Option<bool>,
    #[serde(rename = "UPPERCASE", skip_serializing_if = "Option::is_none")]
    pub uppercase: Option<bool>,
    #[serde(rename = "TIME_COL", skip_serializing_if = "Option::is_none")]
    pub time_color: Option<ColorValue>,
    #[serde(rename = "DATE_COL", skip_serializing_if = "Option::is_none")]
    pub date_color: Option<ColorValue>,
    #[serde(rename = "TEMP_COL", skip_serializing_if = "Option::is_none")]
    pub temp_color: Option<ColorValue>,
    #[serde(rename = "HUM_COL", skip_serializing_if = "Option::is_none")]
    pub humidity_color: Option<ColorValue>,
    #[serde(rename = "BAT_COL", skip_serializing_if = "Option::is_none")]
    pub battery_color: Option<ColorValue>,
    #[serde(rename = "SSPEED", skip_serializing_if = "Option::is_none")]
    pub scroll_speed: Option<u32>,
    #[serde(rename = "TIM", skip_serializing_if = "Option::is_none")]
    pub time_app: Option<bool>,
    #[serde(rename = "DAT", skip_serializing_if = "Option::is_none")]
    pub date_app: Option<bool>,
    #[serde(rename = "HUM", skip_serializing_if = "Option::is_none")]
    pub humidity_app: Option<bool>,
    #[serde(rename = "TEMP", skip_serializing_if = "Option::is_none")]
    pub temp_app: Option<bool>,
    #[serde(rename = "BAT", skip_serializing_if = "Option::is_none")]
    pub battery_app: Option<bool>,
    #[serde(rename = "MATP", skip_serializing_if = "Option::is_none")]
    pub matrix_power: Option<bool>,
    #[serde(rename = "VOL", skip_serializing_if = "Option::is_none")]
    pub volume: Option<u8>,
    #[serde(rename = "OVERLAY", skip_serializing_if = "Option::is_none")]
    pub overlay: Option<OverlayEffect>,
}

