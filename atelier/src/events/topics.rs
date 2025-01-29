use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::{ClientContext, DefaultClientContext};
use rdkafka::config::ClientConfig;

pub async fn create_topic() {
    let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("AdminClient creation failed");

    let new_topic = NewTopic::new("test-topic", 1, TopicReplication::Fixed(1));

    let results = admin_client
        .create_topics(&[new_topic], &AdminOptions::new())
        .await
        .expect("Topic creation failed");

    for result in results {
        match result {
            Ok(topic) => println!("Topic {} created successfully", topic),
            Err((topic, err)) => println!("Failed to create topic {}: {}", topic, err),
        }
    }
}
