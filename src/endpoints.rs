use axum::{extract::State, http::StatusCode, response::Html, Json};

use crate::{
    models::{self},
    web::AppState,
};
use serde_json::json;

#[utoipa::path(get, path = "/", responses((status = OK)))]
pub async fn root() -> Html<&'static str> {
    Html("<h1>YOLO, World!</h1>")
}

#[utoipa::path(get, path = "/city", responses((status = OK, body = Vec<models::City>)))]
pub async fn get_cities(State(state): State<AppState>) -> axum::Json<Vec<models::City>> {
    let cities: Vec<models::City> = sqlx::query_as("SELECT * FROM city")
        .fetch_all(&state.db)
        .await
        .unwrap();

    axum::Json(cities)
}

#[utoipa::path(post, path = "/city", request_body = models::City, responses((status = CREATED, body = models::IdResponse)))]
pub async fn create_city(
    State(state): State<AppState>,
    city: axum::Json<models::City>,
) -> (StatusCode, Json<serde_json::Value>) {
    // let city: models::City = sqlx::query_as(
    let new_id: i32 = sqlx::query_scalar(
        "INSERT INTO city (department_code, insee_code, zip_code, name, lat, lon) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
    )
    .bind(&city.department_code)
    .bind(&city.insee_code)
    .bind(&city.zip_code)
    .bind(&city.name)
    .bind(city.lat)
    .bind(city.lon)
    .fetch_one(&state.db)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(json!({ "id": new_id })))
}

#[utoipa::path(get, path = "/_health", responses((status = NO_CONTENT)))]
pub async fn health_check() -> StatusCode {
    StatusCode::NO_CONTENT
}
