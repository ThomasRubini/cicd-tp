use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, PartialEq, Debug, ToSchema)]
pub struct City {
    pub id: i32,
    pub department_code: String,
    pub insee_code: String,
    pub zip_code: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(serde::Serialize, ToSchema)]
pub struct IdResponse {
    id: i32,
}
