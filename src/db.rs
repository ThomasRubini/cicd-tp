use crate::cli::Args;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;

pub async fn connect(args: &Args) -> Result<sqlx::PgPool, Error> {
    // Database URL
    let database_url: &str = &format!(
        "postgres://{}:{}@{}/{}",
        args.pg_user, args.pg_passwd, args.pg_host, args.pg_db
    );
    println!("Database URL: {}", database_url);

    // Create a connection pool
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
