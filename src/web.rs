use crate::endpoints;
use axum::{routing::get, Router};
use sqlx::Pool;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<sqlx::Postgres>,
}

pub async fn launch_web_server(state: AppState) {
    // build our application with a route
    let app = Router::new()
        .route("/", get(endpoints::root))
        .route(
            "/city",
            get(endpoints::get_cities).post(endpoints::create_city),
        )
        .with_state(state);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
