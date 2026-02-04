use tracing::{debug, error, info};
use crate::services::redis_service::RedisMode;

#[derive(Debug)]
pub struct Config {
    pub redis_url: Option<String>,
    pub sentinel_url: Option<Vec<String>>,
    pub redis_mode: RedisMode,
    pub mongo_url: String,
    pub app_address: String,
    pub app_port: String,
}

impl Config {
    pub fn new(redis_url: Option<String>, sentinel_url: Option<Vec<String>>, redis_mode: RedisMode, mongo_url: String, app_address: String, app_port: String) -> Config {
        Config {
            redis_url,
            redis_mode,
            sentinel_url,
            mongo_url,
            app_address,
            app_port,
        }
    }

    pub fn from_env() -> Config {
        let app_address = std::env::var("SAS_IP").unwrap_or_else(|_| {
            debug!("SAS_IP not specified, using 0.0.0.0");
            String::from("0.0.0.0")
        });

        let app_port = std::env::var("SAS_PORT").unwrap_or_else(|_| {
            debug!("SAS_PORT not specified, using port 8080");
            String::from("8080")
        });

        let redis_mode: RedisMode = match std::env::var("REDIS_USE_SENTINEL") {
            Ok(value) => {
                match value.to_lowercase().as_str() {
                    "true" => RedisMode::Sentinel,
                    _ => RedisMode::Standalone
                }
            }
            Err(_) => {
                info!("REDIS_USE_SENTINEL not set, assuming false");
                RedisMode::Standalone
            }
        };

        let redis_url: Option<String> = match redis_mode {
            RedisMode::Standalone => {
                let redis_string = std::env::var("REDIS_URL").unwrap_or_else(|_| {
                    info!("REDIS_URL not specified, using 'redis://127.0.0.1:6379'");
                    String::from("redis://127.0.0.1:6379")
                });
                Some(redis_string)
            },
            RedisMode::Sentinel => None
        };

        let sentinel_url: Option<Vec<String>> = match redis_mode {
            RedisMode::Standalone => None,
            RedisMode::Sentinel => {
                let sentinel_string = std::env::var("SENTINEL_URL").unwrap_or_else(|_| {
                    error!("SENTINEL_URL not specified!");
                    panic!("SENTINEL_URL not specified!");
                });

                let mut sentinel_vec = Vec::<String>::new();

                for connection_string in sentinel_string.split(',') {
                    sentinel_vec.push(String::from(connection_string));
                }

                Some(sentinel_vec)
            }
        };

        let mongo_url = std::env::var("MONGO_URL").unwrap_or_else(|_| {
            info!("MONGO_URL not specified, using 'mongodb://127.0.0.1:27017'");
            String::from("mongodb://127.0.0.1:27017")
        });

        Config {
            redis_url,
            sentinel_url,
            redis_mode,
            mongo_url,
            app_address,
            app_port,
        }
    }
}