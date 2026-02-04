use tracing::{debug};

#[derive(Debug)]
pub struct Config {
    pub app_address: String,
    pub app_port: String,
}

impl Config {
    pub fn new(app_address: String, app_port: String) -> Config {
        Config {
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

        Config {
            app_address,
            app_port,
        }
    }
}