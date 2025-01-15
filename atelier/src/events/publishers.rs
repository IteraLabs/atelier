// Publishers

use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use std::time::Duration;

pub fn create() -> FutureProducer {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "localhost:9092");

    let producer: FutureProducer = config.create().expect("Failure in message formation");

    producer
}

pub async fn produce(future_producer: FutureProducer, msg: String) {
    let k_topic = "test-topic";
    let record = FutureRecord::to(k_topic)
        .payload(msg.as_str())
        .key("test-key");

    let status_delivery = future_producer
        .send(record, Timeout::After(Duration::from_secs(2)))
        .await;

    match status_delivery {
        Ok(report) => println!("Event transmited {:?}", report),
        Err(e) => println!("Event not transmited, produced this error {:?}", e),
    }
}
