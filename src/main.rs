use crate::config::load_config;
use crate::producer::KafkaProducer;

mod producer;
mod consumer;
mod config;

#[tokio::main]
async fn main() {
    let config = load_config("kafka_config.toml");

    let producer = KafkaProducer::create(&config.kafka);
    producer.send("Message from producer").await;

    consumer::start(&config.kafka).await;
}
