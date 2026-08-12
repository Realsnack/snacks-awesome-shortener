use tracing::debug;

#[derive(Debug)]
pub struct Config {
    pub app_address: String,
    pub app_port: String,
}

impl Config {
    #[must_use]
    pub const fn new(app_address: String, app_port: String) -> Self {
        Self {
            app_address,
            app_port,
        }
    }

    #[must_use]
    pub fn from_env() -> Self {
        let app_address = std::env::var("SAS_IP").unwrap_or_else(|_| {
            debug!("SAS_IP not specified, using 0.0.0.0");
            String::from("0.0.0.0")
        });

        let app_port = std::env::var("SAS_PORT").unwrap_or_else(|_| {
            debug!("SAS_PORT not specified, using port 8080");
            String::from("8080")
        });

        Self {
            app_address,
            app_port,
        }
    }
}
