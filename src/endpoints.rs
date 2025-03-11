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
