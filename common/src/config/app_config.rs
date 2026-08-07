use crate::config::{DbConfig, MessagingConfig};

pub struct Config {
    db_config: DbConfig,
    messaging_config: MessagingConfig,
}

impl Config {
    #[must_use]
    pub fn from_env(cargo_pkg_name: String) -> Self {
        let db_config = DbConfig::from_env();
        let messaging_config = MessagingConfig::from_env(cargo_pkg_name);

        Self {
            db_config,
            messaging_config,
        }
    }

    #[must_use]
    pub const fn get_messaging_config(&self) -> &MessagingConfig {
        &self.messaging_config
    }

    #[must_use]
    pub const fn get_database_config(&self) -> &DbConfig {
        &self.db_config
    }
}
