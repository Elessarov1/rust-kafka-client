use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use crate::config::KafkaConfig;

pub async fn start(kafka_config: &KafkaConfig) {
    let consumer:StreamConsumer = create(&kafka_config);
    handle(&consumer, kafka_config).await
}

fn create(kafka_config: &KafkaConfig) -> StreamConsumer {
    let consumer_config = &kafka_config.consumer;
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", &kafka_config.bootstrap_servers)
        .set("auto.offset.reset", &consumer_config.auto_offset_reset)
        .set("group.id", &consumer_config.group_id)
        .set("socket.timeout.ms", "4000");

    let consumer: StreamConsumer = config.create()
        .expect("Fail to create consumer");

    consumer
}

async fn handle(consumer: &StreamConsumer, config: &KafkaConfig) {
    let topics = [config.topic.as_str()];

    consumer.subscribe(&topics)
        .expect("Fail to subscribe topic");

    loop {
        match consumer.recv().await  {
            Err(e) => println!("{:?}", e),
            Ok(message) => {
                match message.payload_view::<str>() {
                    None => println!("None message"),
                    Some(Ok(message)) => println!("Message recieved: {}", message),
                    Some(Err(e)) => println!("Error on handle message {:?}", e)
                }
                consumer.commit_message(&message, CommitMode::Async).unwrap()
            }
        }
    }
}