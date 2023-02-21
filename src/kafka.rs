use kafka::producer::{ Producer, Record, RequiredAcks };
use kafka::error::Error as KafkaError;
use std::time::Duration;
use std::env;

// pub const KAFKA_TOPIC: &str = "kafkaGpsIncommingDataTopic";
pub const MAX_BUFFER_SIZE: usize = 1024;

pub async fn kafka_send_message(payload: String) -> Result<(), KafkaError> {

    // get environment variables
    let kafka_topic = env::var("KAFKA_TOPIC").unwrap_or("KAFKA_TOPIC does not exists".to_string());
    let kafka_broker = env::var("KAFKA_BROKER").unwrap_or("KAFKA_BROKER does not exists".to_string());

    let mut producer = Producer::from_hosts(vec![kafka_broker.to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    producer
        .send(&Record::from_value(&kafka_topic, payload.as_bytes()))
        .expect("Failed to send message");

    Ok(())
}
