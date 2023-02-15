use std::str;
use std::error::Error;

use crate::model::RawQueclinkData;

pub async fn serialize_message(payload: String) -> Result<String, Box<dyn Error>> {
    println!("tcp_handle_process::serialize_message::data: {:?}", payload.to_string());

    let fields: Vec<&str> = payload.split(",").collect();

    if fields.len() < 3 {
        return Err(From::from("the payload does not have 3 elements"));
    }

    let event_report: Vec<&str> = fields[0].split(":").collect();

    let raw = RawQueclinkData {
        event_report: event_report[1].to_string(),
        raw: payload.to_string(),
        imei: fields[2].to_string(),
    };

    let serialized_raw = serde_json::to_string(&raw).unwrap();
    println!("tcp_handle_process::serialize_message::serialized_data: {:?}", serialized_raw);
    Ok(serialized_raw)
}

pub fn remove_empty_buffer(buffer: String) -> String {
    buffer.replace("\n", "").replace("\0", "").replace("\r", "")
}