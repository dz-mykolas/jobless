use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn connect_sqlx() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv::dotenv().ok();

    // Connect to the database.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
