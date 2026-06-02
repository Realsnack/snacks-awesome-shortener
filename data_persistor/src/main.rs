use async_nats::jetstream::{Context, Message};
use common::Config;
use common::ProtoMessage;
use common::models::messaging::{PersistShortCommand, RetrieveShortCommand, ShortRetrievedEvent};
use common::models::short_url::ShortUrl;
use common::nats_utils::{create_common_headers, create_consumer, get_header_value};
use common::{pg_utils, setup_logging};
use futures_util::TryStreamExt;
use prost::Message as _;
use sqlx::{Pool, Postgres};
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    let config = Config::from_env(env!("CARGO_PKG_NAME").to_string());
    let client = async_nats::connect(&config.get_messaging_config().nats_url).await?;
    let jetstream = async_nats::jetstream::new(client);
    let mut consumer_stream = create_consumer(config.get_messaging_config(), &jetstream).await?;
    let db_pool = pg_utils::create_pool(config.get_database_config()).await?;

    while let Ok(Some(message)) = consumer_stream.try_next().await {
        process_message(&message, db_pool.clone(), &jetstream).await?;
        message.ack().await?;
    }

    Ok(())
}

pub async fn process_message(
    message: &Message,
    db_pool: Pool<Postgres>,
    jetstream: &Context,
) -> Result<(), sqlx::Error> {
    debug!("Message payload: {:?}", &message.message);

    let message_type = get_header_value(&message.message.headers, "message_type").unwrap_or("none");
    let correlation_id = get_header_value(&message.message.headers, "correlation_id")
        .unwrap_or("hehehee")
        .to_string();

    info!("Received {} message", message_type);

    match message_type {
        "PersistShortCommand" => {
            persist_short_command(&message.message.payload, correlation_id, db_pool, jetstream)
                .await?
        }
        "RetrieveShortCommand" => {
            retrieve_short_command(&message.message.payload, correlation_id, db_pool, jetstream)
                .await?
        }
        _ => {
            error!("Unsupported message type '{}'", message_type);
        }
    }

    Ok(())
}

pub async fn persist_short_command(
    message: &[u8],
    _correlation_id: String,
    db_pool: Pool<Postgres>,
    _jetstream: &Context,
) -> Result<(), sqlx::Error> {
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

pub async fn retrieve_short_command(
    message: &[u8],
    correlation_id: String,
    db_pool: Pool<Postgres>,
    jetstream: &Context,
) -> Result<(), sqlx::Error> {
    let decoded_payload =
        common::proto::messaging::v1::commands::RetrieveShortCommand::decode(message).unwrap();
    debug!("message received: {:?}", decoded_payload);
    let converted_payload = RetrieveShortCommand::from(decoded_payload);
    info!("Decoded message received: {:?}", converted_payload);
    // TODO: Retrieve short from redis

    let result = sqlx::query!(
        r#"SELECT * FROM retrieve_short($1);"#,
        converted_payload.short_url
    )
    .fetch_one(&db_pool)
    .await?;
    debug!("Db result: {:?}", result);

    let headers =
        create_common_headers(String::from("ShortRetrievedEvent"), correlation_id.clone());

    let retrieved_short = ShortUrl::new(
        result.short_url.unwrap(),
        result.long_url.unwrap(),
        result.expiration.unwrap().into(),
    );

    let instance_id = std::env::var("HOSTNAME").unwrap_or("unknown".into());

    debug!("Retrieved short: {:?}", retrieved_short);

    let response = ShortRetrievedEvent::new(retrieved_short, instance_id);

    match jetstream
        .publish_with_headers(
            "api_gateway::response",
            headers,
            response.to_proto().encode_to_vec().into(),
        )
        .await
    {
        Ok(_) => info!(
            "Response sent successfully. correlation_id: {}",
            correlation_id
        ),
        Err(e) => error!(
            "Unable to send message. correlation_id: {}\n{}",
            correlation_id, e
        ),
    };
    jetstream.client().flush().await.unwrap();

    Ok(())
}
