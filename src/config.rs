use tracing::{debug, info};

pub struct Config {
    pub redis_url: String,
    pub mongo_url: String,
    pub app_address: String,
    pub app_port: String,
}

impl Config {
    pub fn from_env() -> Config {
        let app_address = std::env::var("SAS_IP").unwrap_or_else(|_| {
            debug!("SAS_IP not specified, using 0.0.0.0");
            String::from("0.0.0.0")
        });

        let app_port = std::env::var("SAS_PORT").unwrap_or_else(|_| {
            debug!("SAS_PORT not specified, using port 8080");
            String::from("8080")
        });

        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| {
            info!("REDIS_URL not specified, using 'redis://127.0.0.1:6379'");
            String::from("redis://127.0.0.1:6379")
        });

        let mongo_url = std::env::var("MONGO_URL").unwrap_or_else(|_| {
            info!("MONGO_URL not specified, using 'mongodb://127.0.0.1:27017'");
            String::from("mongodb://127.0.0.1:27017")
        });

        Config {
            redis_url,
            mongo_url,
            app_address,
            app_port,
        }
    }
}