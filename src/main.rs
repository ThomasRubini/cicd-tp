pub mod cli;
pub mod db;
pub mod endpoints;
pub mod models;
pub mod web;

#[tokio::main]
async fn main() {
    println!("Starting program");

    let args = cli::parse_args();
    let db_pool = db::connect(args).await.unwrap();
    let state = web::AppState { db: db_pool };

    web::launch_web_server(state).await;
}
