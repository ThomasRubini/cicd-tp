use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt; // for `collect` method
use sqlx::PgPool;
use tower::ServiceExt;

use crate::{
    models,
    web::{self, AppState},
}; // for `oneshot` method

async fn create_router(state: AppState) -> Router {
    web::create_router(state).await.split_for_parts().0
}

async fn fake_state() -> AppState {
    let db_url = std::env::var("TEST_DB_URL")
        .unwrap_or_else(|_| "postgres://cicd:cicd@localhost:5432/cicd".to_string());
    AppState {
        db: sqlx::Pool::connect(&db_url).await.unwrap(),
    }
}

#[tokio::test]
async fn test_health_check() {
    let app = create_router(fake_state().await).await;

    let request = Request::builder()
        .uri("/_health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[sqlx::test]
async fn test_insert_city(pool: PgPool) {
    let state = AppState { db: pool };
    let app = create_router(state.clone()).await;

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

#[sqlx::test]
async fn test_get_city(pool: PgPool) {
    let state = AppState { db: pool };
    let app = create_router(state.clone()).await;

    let city = models::City {
        id: 1,
        department_code: "NY".to_string(),
        insee_code: "NY".to_string(),
        zip_code: "NY".to_string(),
        name: "New York".to_string(),
        lat: 40.7128,
        lon: -74.0060,
    };

    // Insert the city into the database
    sqlx::query(
        "INSERT INTO city (department_code, insee_code, zip_code, name, lat, lon) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
    )
    .bind(&city.department_code)
    .bind(&city.insee_code)
    .bind(&city.zip_code)
    .bind(&city.name)
    .bind(city.lat)
    .bind(city.lon).execute(&state.db).await.unwrap();

    let request = Request::builder().uri("/city").body(Body::empty()).unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let db_cities: Vec<models::City> = serde_json::from_slice(&body).unwrap();
    assert_eq!(db_cities, vec![city]);
}
