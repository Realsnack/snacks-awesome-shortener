use async_nats::jetstream::Message;
use common::models::messaging::PersistShortCommand;
use common::nats_utils::create_consumer;
use common::{db_config::DbConfig, messaging_config::MessagingConfig};
use common::{pg_utils, setup_logging};
use futures_util::TryStreamExt;
use prost::Message as _;
use sqlx::{Pool, Postgres};
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    let config = MessagingConfig::from_env(env!("CARGO_PKG_NAME").to_string());
    let mut consumer_stream = create_consumer(&config).await?;
    let db_config = DbConfig::from_env();
    let db_pool = pg_utils::create_pool(db_config).await?;

    while let Ok(Some(message)) = consumer_stream.try_next().await {
        process_message(&message, db_pool.clone()).await?;
        message.ack().await?;
    }

    Ok(())
}

pub async fn process_message(
    message: &Message,
    db_pool: Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    debug!("Message payload: {:?}", &message.message);

    let message_type = match &message.headers {
        None => {
            error!("No headers in message: {:?}", &message);
            "none"
        }
        Some(headers) => match headers.get("message_type") {
            None => {
                error!(
                    "No 'message_type' header in message: {:?}",
                    &message.message
                );
                "none"
            }
            Some(message_type) => message_type.as_str(),
        },
    };

    info!("Received {} message", message_type);

    match message_type {
        "PersistShortCommand" => persist_short_command(&message.message.payload, db_pool).await?,
        _ => {
            error!("Unsupported message type '{}'", message_type);
        }
    }

    Ok(())
}

pub async fn persist_short_command(
    message: &[u8],
    db_pool: Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    // TODO: Decode message
    let decoded_payload =
        common::proto::messaging::v1::commands::PersistShortCommand::decode(message).unwrap();
    debug!("message received: {:?}", decoded_payload);
    let converted_payload = PersistShortCommand::from(decoded_payload);
    info!("Decoded message received: {:?}", converted_payload);

    // TODO: Save to redis
    let created = converted_payload.created as f64;

    match sqlx::query!(
        r#"
    INSERT INTO shorts(short_url, long_url, expiration, created)
    VALUES ($1, $2, $3, to_timestamp($4));
    "#,
        converted_payload.short.short_url,
        converted_payload.short.long_url,
        converted_payload.short.expiration,
        created,
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => {
            info!(
                "Short {} written to database",
                converted_payload.short.short_url
            );
            Ok(())
        }
        Err(e) => {
            error!(
                "Unable to write short '{}' to postgres due to error: {}",
                converted_payload.short.short_url, e
            );
            Ok(())
        }
    }
}
