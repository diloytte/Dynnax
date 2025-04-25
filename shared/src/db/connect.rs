use sqlx::postgres::PgPoolOptions;

use super::Database;

pub async fn connect(db_url: String) -> Result<Database, sqlx::error::Error> {
    let pool: Database = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await?;
    Ok(pool)
}
