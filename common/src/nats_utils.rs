use async_nats::jetstream::Context;
use async_nats::jetstream::consumer::Consumer;
use async_nats::jetstream::context::CreateStreamError;
use async_nats::jetstream::stream::{Config, ConsumerError, Stream};
use futures_util::StreamExt;
use futures_util::stream::Take;

pub async fn create_consumer(
    config: &crate::messaging_config::MessagingConfig,
) -> Result<Take<async_nats::jetstream::consumer::pull::Stream>, async_nats::Error> {
    let client = async_nats::connect(&config.nats_url).await?;
    let jetstream = async_nats::jetstream::new(client);

    let request_stream = get_stream(
        &jetstream,
        config.request_stream.clone(),
        config.request_stream_max_messages,
    )
    .await?;

    let consumer = create_pull_consumer(
        request_stream,
        config.consumer_name.clone(),
        config.request_stream.clone(),
    )
    .await?;

    let messages = consumer.messages().await?.take(100);

    Ok(messages)
}

pub async fn get_stream(
    jetstream: &Context,
    stream_name: String,
    stream_max_messages: i64,
) -> Result<Stream, CreateStreamError> {
    jetstream
        .get_or_create_stream(Config {
            name: stream_name,
            max_messages: stream_max_messages,
            ..Default::default()
        })
        .await
}

pub async fn create_pull_consumer(
    request_stream: Stream,
    consumer_name: String,
    durable_name: String,
) -> Result<Consumer<async_nats::jetstream::consumer::pull::Config>, ConsumerError> {
    request_stream
        .get_or_create_consumer(
            consumer_name.as_str(),
            async_nats::jetstream::consumer::pull::Config {
                durable_name: Some(durable_name),
                ..Default::default()
            },
        )
        .await
}
