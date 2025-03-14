use crate::endpoints;
use axum::{routing::get, Router};
use sqlx::Pool;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<sqlx::Postgres>,
}

pub async fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(endpoints::root))
        .route(
            "/city",
            get(endpoints::get_cities).post(endpoints::create_city),
        )
        .route("/_health", get(endpoints::health_check))
        .with_state(state)
}

pub async fn launch(router: Router) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
