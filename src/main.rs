pub mod web;
pub mod models;
pub mod endpoints;
pub mod db;
pub mod cli;


#[tokio::main]
async fn main() {
    println!("Starting program");

    let args = cli::parse_args();
    let db_pool = db::connect(args).await.unwrap();
    let state = web::AppState {
        db: db_pool
    };

    web::launch_web_server(state).await;
}
