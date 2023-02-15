use kafka::producer::{ Producer, Record, RequiredAcks };
use kafka::error::Error as KafkaError;
use std::time::Duration;

pub const KAFKA_TOPIC: &str = "kafkaGpsIncommingDataTopic";
pub const MAX_BUFFER_SIZE: usize = 1024;

pub async fn kafka_send_message(payload: String) -> Result<(), KafkaError> {
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