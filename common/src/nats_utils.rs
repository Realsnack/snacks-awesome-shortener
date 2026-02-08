use async_nats::jetstream::{Context, Message};
use async_nats::jetstream::consumer::Consumer;
use async_nats::jetstream::context::CreateStreamError;
use async_nats::jetstream::stream::{Config, ConsumerError, Stream};
use futures_util::{StreamExt, TryStreamExt};

pub async fn create_consumer(
        config: crate::config::Config,
        process_message_func: fn(&Message)) -> Result<(), async_nats::Error> {
    let client = async_nats::connect(config.nats_url).await?;
    let jetstream = async_nats::jetstream::new(client);

    let request_stream = get_stream(&jetstream,
                                    config.request_stream.clone(),
                                    config.request_stream_max_messages).await?;

    let consumer = create_pull_consumer(request_stream,
                                        config.consumer_name,
                                        config.request_stream).await?;

    let mut messages = consumer.messages().await?.take(100);
    while let Ok(Some(message)) = messages.try_next().await {
        process_message_func(&message);
        message.ack().await?;
        jetstream.publish(config.response_stream.clone(), "Confirm".into()).await?;
    }

    Ok(())
}

pub async fn get_stream(jetstream: &Context, stream_name: String, stream_max_messages: i64) -> Result<Stream, CreateStreamError> {
    jetstream
        .get_or_create_stream(Config {
            name: stream_name,
            max_messages: stream_max_messages,
            ..Default::default()
        }).await
}

pub async fn create_pull_consumer(request_stream: Stream, consumer_name: String, durable_name: String) -> Result<Consumer<async_nats::jetstream::consumer::pull::Config>, ConsumerError> {
    request_stream
        .get_or_create_consumer(
            consumer_name.as_str(),
            async_nats::jetstream::consumer::pull::Config {
                durable_name: Some(durable_name),
                ..Default::default()
            },
        ).await
}