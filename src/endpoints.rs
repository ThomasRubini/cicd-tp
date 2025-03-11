use axum::{extract::State, response::Html};

use crate::{
    models::{self},
    web::AppState,
};

pub async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn get_cities(State(state): State<AppState>) -> axum::Json<Vec<models::City>> {
    let cities: Vec<models::City> = sqlx::query_as("SELECT * FROM city")
        .fetch_all(&state.db)
        .await
        .unwrap();

    axum::Json(cities)
}

pub async fn create_city(
    State(state): State<AppState>,
    city: axum::Json<models::City>,
) -> axum::Json<models::City> {
    // let city: models::City = sqlx::query_as(
    let city: i32 = sqlx::query_scalar(
        "INSERT INTO city (department_code, insee_code, zip_code, name, lat, lon) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
    )
    .bind(&city.department_code)
    .bind(&city.insee_code)
    .bind(&city.zip_code)
    .bind(&city.name)
    .bind(&city.lat)
    .bind(&city.lon)
    .fetch_one(&state.db)
    .await
    .unwrap();

    println!("{}", city);

    todo!();

    // axum::Json(city)
}