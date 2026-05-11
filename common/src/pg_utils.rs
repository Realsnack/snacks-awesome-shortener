use sqlx::{self, Pool, Postgres, postgres::PgPoolOptions};

use crate::db_config::DbConfig;

pub async fn create_pool(db_config: DbConfig) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(db_config.max_connections)
        .connect(
            format!(
                "postgres://{}:{}@{}/{}",
                db_config.username, db_config.password, db_config.host, db_config.database_name
            )
            .as_str(),
        )
        .await?;

    Ok(pool)
}
