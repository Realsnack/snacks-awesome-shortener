use common::models::persistence_request::PersistenceRequest;
use futures_util::{StreamExt, TryStreamExt};

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("localhost:4222").await?;
    let jetstream = async_nats::jetstream::new(client);

    let request_stream = jetstream
        .get_or_create_stream(async_nats::jetstream::stream::Config {
            name: "data_persistor::request".to_string(),
            max_messages: 1_000,
            ..Default::default()
        }).await?;

    let consumer = request_stream
        .get_or_create_consumer(
            "data_persistor",
            async_nats::jetstream::consumer::pull::Config {
                durable_name: Some("data_persistor::request".to_string()),
                ..Default::default()
            },
        ).await?;

    let mut messages = consumer.messages().await?.take(100);
    while let Ok(Some(message)) = messages.try_next().await {
        let decoded_payload = PersistenceRequest::from_vec(&message.message.payload.to_vec());
        println!("message received: {:?}", decoded_payload);
        message.ack().await?;

        jetstream.publish("data_persistor::response", "Confirm".into()).await?;
    }

    Ok(())
}
