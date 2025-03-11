use sqlx::postgres::PgPoolOptions;
use sqlx::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Database URL
    let database_url = "postgres://user:password@localhost/yourdbname";

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(())
}