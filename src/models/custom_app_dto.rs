use serde::{Deserialize, Serialize};
use crate::ColorValue;
use crate::models::draw::Draw;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum TextCase {
    Global = 0,
    ForceUppercase = 1,
    AsIs = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PushIcon {
    Static = 0,
    ScrollOnce = 1,
    ScrollLoop = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LifetimeMode {
    Delete = 0,
    Stale = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum GradientValue {
    Hex(Vec<String>),
    Int(Vec<i32>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TextValue {
    String(String),
    Colored(Vec<TextColorValue>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextColorValue {
    #[serde(rename = "t")]
    pub text : String,
    #[serde(rename = "c")]
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CustomAppDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextValue>,

    #[serde(rename = "textCase", skip_serializing_if = "Option::is_none")]
    pub text_case: Option<TextCase>,

    #[serde(rename = "topText", skip_serializing_if = "Option::is_none")]
    pub top_text: Option<bool>,

    #[serde(rename = "textOffset", skip_serializing_if = "Option::is_none")]
    pub text_offset: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<GradientValue>,

    #[serde(rename = "blinkText", skip_serializing_if = "Option::is_none")]
    pub blink_text: Option<i32>,

    #[serde(rename = "fadeText", skip_serializing_if = "Option::is_none")]
    pub fade_text: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ColorValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rainbow: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(rename = "pushIcon", skip_serializing_if = "Option::is_none")]
    pub push_icon: Option<PushIcon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtttl: Option<String>,

    #[serde(rename = "loopSound", skip_serializing_if = "Option::is_none")]
    pub loop_sound: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<Vec<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<bool>,

    #[serde(rename = "barBC", skip_serializing_if = "Option::is_none")]
    pub bar_bc: Option<ColorValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,

    #[serde(rename = "progressC", skip_serializing_if = "Option::is_none")]
    pub progress_c: Option<ColorValue>,

    #[serde(rename = "progressBC", skip_serializing_if = "Option::is_none")]
    pub progress_bc: Option<ColorValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw: Option<Vec<Draw>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime: Option<i32>,

    #[serde(rename = "lifetimeMode", skip_serializing_if = "Option::is_none")]
    pub lifetime_mode: Option<LifetimeMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wakeup: Option<bool>,

    #[serde(rename = "noScroll", skip_serializing_if = "Option::is_none")]
    pub no_scroll: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients: Option<Vec<String>>,

    #[serde(rename = "scrollSpeed", skip_serializing_if = "Option::is_none")]
    pub scroll_speed: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,

    #[serde(rename = "effectSettings", skip_serializing_if = "Option::is_none")]
    pub effect_settings: Option<EffectSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlay: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EffectSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub palette: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend: Option<bool>,
}