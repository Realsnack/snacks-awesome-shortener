use crate::config::{DbConfig, MessagingConfig};

pub struct Config {
    db_config: DbConfig,
    messaging_config: MessagingConfig,
}

impl Config {
    pub fn from_env(cargo_pkg_name: String) -> Self {
        let db_config = DbConfig::from_env();
        let messaging_config = MessagingConfig::from_env(cargo_pkg_name);

        Self {
            db_config,
            messaging_config,
        }
    }
}
