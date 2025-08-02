use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub kafka: KafkaConfig
}

#[derive(Debug, Deserialize)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub topic: String,
    pub consumer: ConsumerConfig,
    pub producer: ProducerConfig
}

#[derive(Debug, Deserialize)]
pub struct ConsumerConfig {
    pub group_id: String,
    pub auto_offset_reset: String
}

#[derive(Debug, Deserialize)]
pub struct ProducerConfig {
    pub message_timeout: String,
    pub acks_mode: String,
    pub retries: String
}

pub fn load_config(path: &str) -> AppConfig {
    let content = fs::read_to_string(path)
        .expect("Can't read config file");

    toml::from_str(&content).expect("Invalid TOML format")
}