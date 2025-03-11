use sqlx::PgPool;

pub mod web;
pub mod models;
pub mod endpoints;
pub mod db;
pub mod cli;


#[tokio::main]
async fn main() {
    println!("Starting program");

    let db_pool = PgPool::connect("postgres://cicd:cicd@localhost/cicd")
        .await
        .expect("Failed to connect to Postgres");

    let state = web::AppState {
        db: db_pool
    };

    web::launch_web_server(state).await;
}
