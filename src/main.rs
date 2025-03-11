mod web;


#[tokio::main]
async fn main() {
    println!("Starting program");

    web::launch_web_server().await;
}
