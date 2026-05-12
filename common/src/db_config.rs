//! Represents the configuration to connect to PostgreSQL database using sqlx
use tracing::{info, warn};

/// Simple datbase configuraion struct
#[derive(Debug)]
pub struct DbConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub database_name: String,
    pub max_connections: u32,
}

impl DbConfig {
    pub fn new(
        username: String,
        password: String,
        host: String,
        database_name: String,
        max_connections: u32,
    ) -> DbConfig {
        DbConfig {
            username,
            password,
            host,
            database_name,
            max_connections,
        }
    }

    /// Function to fetch configuration form ENV variables
    /// | Variable           | Description                              | Default   |
    /// |--------------------|------------------------------------------|-----------|
    /// | DB_USERNAME        | Username used to conenct to PostgreSQL   | sas_app   |
    /// | DB_PASSWORD        | Password to connect to PostgreSQL        | sas_pass  |
    /// | DB_HOST            | Hostname/DNS of the server to connect to | 127.0.0.1 |
    /// | DB_NAME            | Postgres Database name                   | sas_db    |
    /// | DB_MAX_CONNECTIONS | Maximum connections in pool              | 100       |
    pub fn from_env() -> DbConfig {
        let username = std::env::var("DB_USERNAME").unwrap_or_else(|_| {
            let username = String::from("sas_app");
            warn!("No DB_USERNAME configured, using '{}'", username);
            username
        });

        let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| {
            let password = String::from("sas_pass");
            warn!("No DB_PASSWORD configured, using default password");
            password
        });

        let host = std::env::var("DB_HOST").unwrap_or_else(|_| {
            let host = String::from("127.0.0.1");
            warn!("No DB_HOST configured, using '{}'", host);
            host
        });

        let database_name = std::env::var("DB_NAME").unwrap_or_else(|_| {
            let database_name = String::from("sas_db");
            info!("No DB_NAME configured, using '{}'", database_name);
            String::from("localhost:4222")
        });

        let max_connections = std::env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| {
                warn!("No DB_MAX_CONNECTIONS configured, using 1000 as default");
                String::from("1000")
            })
            .parse()
            .unwrap_or_else(|_| {
                warn!("Cannot parse DB_MAX_CONNECTIONS to isize");
                100
            });

        DbConfig {
            username,
            password,
            host,
            database_name,
            max_connections,
        }
    }
}
