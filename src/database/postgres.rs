use std::time::Duration;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let database_url = dotenvy::var("DATABASE_URL").expect("No `DATABASE_URL` found in .env");
    let pool = PgPoolOptions::new()
        .min_connections(0)
        .max_connections(16)
        .max_lifetime(Some(Duration::from_secs(60 * 60)))
        .connect(&database_url)
        .await?;

    Ok(pool)
}