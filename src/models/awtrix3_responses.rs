use serde_json::Map;

pub type StringVec = std::vec::Vec<String>;
pub type LoopResponse = std::collections::HashMap<String, u64>;
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct StatsResponse {
    pub bat: i32,
    pub bat_raw: i32,
    #[serde(rename = "type")]
    pub device_type: i32,
    pub lux: i32,
    pub ldr_raw: i32,
    pub ram: i32,
    pub bri: i32,
    pub temp: i32,
    pub hum: i32,
    pub uptime: u64,
    pub wifi_signal: i32,
    pub messages: i32,
    pub version: String,
    pub indicator1: bool,
    pub indicator2: bool,
    pub indicator3: bool,
    pub app: String,
    pub uid: String,
    pub matrix: bool,
    pub ip_address: String,
}
