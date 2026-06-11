mod custom_app_dto;
mod moodlight_dto;
mod colored_indicator_dto;
mod draw;
mod settings_dto;
mod awtrix3_responses;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ColorValue {
    Hex(String),
    Int(i32),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum IndicatorType {
    One = 1,
    Two = 2,
    Three = 3,
}

impl IndicatorType {
    pub fn to_int(&self) -> i8 {
        match self {
            IndicatorType::One => 1,
            IndicatorType::Two => 2,
            IndicatorType::Three => 3,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppSwitchMode {
    Next,
    Previous,
}
impl AppSwitchMode {
    pub fn to_route(&self) -> String {
        match self {
            AppSwitchMode::Next => "nextapp".to_string(),
            AppSwitchMode::Previous => "previousapp".to_string(),
        }
    }
}

use serde::{Deserialize, Serialize};
pub use custom_app_dto::CustomAppDto as CustomAppDto;
pub use custom_app_dto::GradientValue as GradientValue;
pub use custom_app_dto::EffectSettings as EffectSettings;
pub use custom_app_dto::TextCase as TextCase;
pub use custom_app_dto::PushIcon as PushIcon;
pub use custom_app_dto::LifetimeMode as LifetimeMode;
pub use custom_app_dto::TextColorValue as TextColorValue;
pub use custom_app_dto::TextValue as TextValue;
pub use draw::Draw as Draw;
pub use draw::DrawRGB as DrawRGB;
pub use draw::DrawText as DrawText;
pub use draw::DrawLine as DrawLine;
pub use draw::DrawObject as DrawObject;
pub use draw::DrawPixel as DrawPixel;
pub use moodlight_dto::MoodLightingDto as MoodLightingDto;
pub use colored_indicator_dto::ColoredIndicatorDto as ColoredIndicatorDto;
pub use settings_dto::SettingsDto as SettingsDto;
pub use settings_dto::TransitionEffect as TransitionEffect;
pub use settings_dto::TimeMode as TimeMode;
pub use settings_dto::OverlayEffect as OverlayEffect;
pub use awtrix3_responses::StringVec as Awtrix3StringVecResponse;
pub use awtrix3_responses::StatsResponse as Awtrix3StatsResponse;
pub use awtrix3_responses::LoopResponse as Awtrix3LoopResponse;