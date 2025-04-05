pub mod cli;
pub mod db;
pub mod endpoints;
pub mod models;
#[cfg(test)]
pub mod tests;
pub mod web;

#[tokio::main]
async fn main() {
    println!("Starting program");

    let args = cli::parse_args();

    println!("Connecting to database..");
    let db_pool = db::connect(&args).await.unwrap();
    println!("Connected to database");

    let state = web::AppState { db: db_pool };

    web::launch(web::create_full_router(state).await, args.bind_address).await;
}
