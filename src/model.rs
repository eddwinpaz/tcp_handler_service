use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct RawQueclinkData {
    pub event_report: String,
    pub raw: String,
    pub imei: String,
}