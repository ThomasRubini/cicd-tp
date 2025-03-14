use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect` method
use tower::ServiceExt;

use crate::{
    models,
    web::{self, AppState},
}; // for `oneshot` method

async fn fake_state() -> AppState {
    let db_url = std::env::var("TEST_DB_URL")
        .unwrap_or_else(|_| "postgres://cicd:cicd@localhost:5432/cicd".to_string());
    AppState {
        db: sqlx::Pool::connect(&db_url).await.unwrap(),
    }
}

#[tokio::test]
async fn test_health_check() {
    let app = web::create_router(fake_state().await).await;

    let request = Request::builder()
        .uri("/_health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_insert_city() {
    let state = fake_state().await;
    let app = web::create_router(state.clone()).await;

    let mut city = models::City {
        id: 1,
        department_code: "NY".to_string(),
        insee_code: "NY".to_string(),
        zip_code: "NY".to_string(),
        name: "New York".to_string(),
        lat: 40.7128,
        lon: -74.0060,
    };

    let request = Request::builder()
        .uri("/city")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&city).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Get ID from response
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = serde_json::from_slice::<serde_json::Value>(&body).unwrap();
    let id = body["id"].as_i64().unwrap() as i32;

    // Check if the city was inserted
    let db_city: models::City = sqlx::query_as("SELECT * FROM city WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .unwrap();

    city.id = id;
    assert_eq!(db_city, city);
}
