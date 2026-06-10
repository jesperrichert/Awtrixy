use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Draw {
    #[serde(rename = "dp", skip_serializing_if = "Option::is_none")]
    pub pixel : Option<Vec<DrawPixel>>,
    #[serde(rename = "dl", skip_serializing_if = "Option::is_none")]
    pub line : Option<Vec<DrawLine>>,
    #[serde(rename = "dr", skip_serializing_if = "Option::is_none")]
    pub rectangle : Option<Vec<DrawObject>>,
    #[serde(rename = "df", skip_serializing_if = "Option::is_none")]
    pub filled_rectangle : Option<Vec<DrawObject>>,
    #[serde(rename = "dc", skip_serializing_if = "Option::is_none")]
    pub circle : Option<Vec<DrawObject>>,
    #[serde(rename = "dfc", skip_serializing_if = "Option::is_none")]
    pub filled_circle : Option<Vec<DrawObject>>,
    #[serde(rename = "dt", skip_serializing_if = "Option::is_none")]
    pub text : Option<Vec<DrawText>>,
    #[serde(rename = "db", skip_serializing_if = "Option::is_none")]
    pub rgb : Option<Vec<DrawRGB>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DrawPixel {
    pub x : i16,
    pub y : i16,
    pub cl : String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DrawLine {
    pub x0 : i16,
    pub y0: i16,
    pub x1 : i16,
    pub y1: i16,
    pub cl : String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DrawObject {
    pub x : Option<i16>,
    pub y : Option<i16>,
    pub w : Option<i16>,
    pub h : Option<i16>,
    pub r : Option<i16>,
    pub cl : String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DrawText {
    pub x : Option<i16>,
    pub y : Option<i16>,
    pub t: String,
    pub cl : String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DrawRGB {
    pub x : Option<i16>,
    pub y : Option<i16>,
    pub w : Option<i16>,
    pub h : Option<i16>,
    pub bmp : Vec<u8>,
}