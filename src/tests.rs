use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use crate::web::{self, AppState}; // for `oneshot` method

async fn fake_state() -> AppState {
    let db_url = std::env::var("TEST_DB_URL")
        .unwrap_or_else(|_| "postgres://cicd:cicd@localhost:5432/cicd".to_string());
    AppState {
        db: sqlx::Pool::connect(&db_url).await.unwrap(),
    }
}

#[tokio::test]
async fn test_hello_world() {
    let app = web::create_router(fake_state().await).await;

    let request = Request::builder()
        .uri("/_health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
