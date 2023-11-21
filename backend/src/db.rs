use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn connect_sqlx() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv::dotenv().ok();

    // Get the database URL from the .env file.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if database_url.contains("placeholder_") {
        panic!("DATABASE_URL in .env file is still a placeholder. Please update it with your actual database credentials.");
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
