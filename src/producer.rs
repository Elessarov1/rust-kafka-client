use std::time::Duration;
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use crate::config::KafkaConfig;

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub async fn send(&self, message: &str) {
        let record = FutureRecord::to(&self.topic)
            .payload(message)
            .key("Part-Key");

        let result = self.producer
            .send(record, Timeout::After(Duration::from_secs(2)))
            .await;

        match result {
            Ok(report) => println!("Message sent: {:?}", report),
            Err(e) => println!("Error sending message: {:?}", e),
        }
    }

    pub fn create(kafka_config: &KafkaConfig) -> Self {
        Self::create_topic(kafka_config);

        let producer_config = &kafka_config.producer;
        let mut client_config = ClientConfig::new();

        client_config
            .set("bootstrap.servers", &kafka_config.bootstrap_servers)
            .set("message.timeout.ms", &producer_config.message_timeout)
            .set("acks", &producer_config.acks_mode)
            .set("retries", &producer_config.retries);

        let producer = client_config
            .create::<FutureProducer>()
            .expect("Failed to create Kafka producer");

        Self {
            producer,
            topic: kafka_config.topic.clone(),
        }
    }

    fn create_topic(config: &KafkaConfig) {
        let mut admin_config = ClientConfig::new();
        admin_config.set("bootstrap.servers", &config.bootstrap_servers);

        let kafka_admin: AdminClient<_> = admin_config
            .create()
            .expect("Failed to create Kafka AdminClient");

        let topic = &config.topic;
        let new_topic = NewTopic::new(topic, 1, TopicReplication::Fixed(1));
        let topics = [new_topic];

        match futures::executor::block_on(kafka_admin.create_topics(&topics, &AdminOptions::new())) {
            Ok(results) => {
                for result in results {
                    match result {
                        Ok(topic) => println!("Topic '{}' created", topic),
                        Err((topic, error)) => {
                            println!("Failed to create topic '{}': {}", topic, error);
                        }
                    }
                }
            }
            Err(e) => println!("Kafka admin error: {}", e),
        }
    }
}