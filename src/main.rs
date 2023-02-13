#![warn(rust_2018_idioms)]

use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::{ TcpListener, TcpStream };
use std::error::Error;
use std::str;
use std::time::Duration;
use kafka::producer::{ Producer, Record, RequiredAcks };
use kafka::error::Error as KafkaError;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct RawQueclinkData {
    event_report: String,
    raw: String,
    imei: String,
}
const KAFKA_TOPIC: &str = "kafkaGpsIncommingDataTopic";
const MAX_BUFFER_SIZE: usize = 1024;

async fn kafka_send_message(payload: String) -> Result<(), KafkaError> {
    let mut producer = Producer::from_hosts(vec!["3.137.212.86:9092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    producer
        .send(&Record::from_value(KAFKA_TOPIC, payload.as_bytes()))
        .expect("Failed to send message");

    Ok(())
}

async fn serialize_message(payload: String) -> Result<String, Box<dyn Error>> {
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

fn remove_empty_buffer(buffer: String) -> String {
    buffer.replace("\n", "").replace("\0", "").replace("\r", "")
}

async fn tcp_handle_process(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf: [u8; MAX_BUFFER_SIZE] = [0; MAX_BUFFER_SIZE];
    loop {
        let n = socket
            .read(&mut buf).await
            .expect("tcp_handle_process::failed_to_read_data_from_socket");
        if n == 0 {
            return Ok(());
        }

        let raw_buffer_data = str::from_utf8(&buf).unwrap();

        println!("tcp_handle_process::received_data: {}", raw_buffer_data);

        let cleaned_buffer: String = remove_empty_buffer(raw_buffer_data.to_string());
        let serialized_raw = serialize_message(cleaned_buffer).await;

        match serialized_raw {
            Ok(gps_json_data) => {
                println!("tcp_handle_process::serialize_message::success");
                let kafka_process = kafka_send_message(gps_json_data).await;

                match kafka_process {
                    Ok(_) => {
                        println!("tcp_handle_process::kafka_send_message::success");
                        socket
                            .write_all(b"+RESP:GBTMI").await
                            .expect("failed to write data to socket");
                    }
                    Err(error) => {
                        println!("tcp_handle_process::kafka_send_message::error: {}", error);
                        socket
                            .write_all(b"+RESP:GBTMI").await
                            .expect("failed to write data to socket");
                    }
                }
            }
            Err(error) => {
                println!("tcp_handle_process::serialize_message::error: {}", error);
            }
        }
        // stream.write_all(&buf[0..n]).await?;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let port = env::var("SERVER_PORT").is_ok();
    // let port = match port {
    //     true => env::var("SERVER_PORT").unwrap(),
    //     false => "7011".to_string(),
    // };
    let host = String::from("0.0.0.0");
    let addr = format!("{}:7011", host);

    let listener = TcpListener::bind(&addr).await?;
    println!("Server is listening: {}", addr);

    let (socket, _) = listener.accept().await?;
    tcp_handle_process(socket).await?;
    Ok(())
}
