use tracing::{info, warn};

#[derive(Debug)]
pub struct Config {
    pub request_stream: String,
    pub response_stream: String,
    pub consumer_name: String,
    pub nats_url: String,
    pub request_stream_max_messages: i64,
}

impl Config {
    pub fn new(
        request_stream: String,
        response_stream: String,
        consumer_name: String,
        nats_url: String,
        request_stream_max_messages: i64,
    ) -> Config {
        Config {
            request_stream,
            response_stream,
            consumer_name,
            nats_url,
            request_stream_max_messages,
        }
    }

    pub fn from_env(cargo_pkg_name: String) -> Config {
        let request_stream = std::env::var("REQUEST_STREAM").unwrap_or_else(|_| {
            info!("No REQUEST_STREAM configured");
            format!("{}::request", cargo_pkg_name)
        });

        let response_stream = std::env::var("RESPONSE_STREAM").unwrap_or_else(|_| {
            info!("No RESPONSE_STREAM configured");
            format!("{}::response", cargo_pkg_name)
        });

        let consumer_name = std::env::var("CONSUMER_NAME").unwrap_or(cargo_pkg_name);

        let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| {
            info!("No NATS_URL configured, using localhost:4222");
            String::from("localhost:4222")
        });

        let max_messages = std::env::var("REQUEST_MAX_MESSAGES").unwrap_or_else(|_| {
            warn!("No REQUEST_MAX_MESSAGES configured - using 1000 as default");
            String::from("1000")
        });

        let request_stream_max_messages = max_messages
            .parse()
            .expect("Couldn't convert REQUEST_MAX_MESSAGES");

        Config {
            request_stream,
            response_stream,
            consumer_name,
            nats_url,
            request_stream_max_messages,
        }
    }
}
