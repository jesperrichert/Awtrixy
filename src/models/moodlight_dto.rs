use serde::{Deserialize, Serialize};
use crate::ColorValue;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MoodLightingDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness : Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kelvin : Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorValue>,
}