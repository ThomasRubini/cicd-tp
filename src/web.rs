use crate::endpoints;
use sqlx::Pool;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<sqlx::Postgres>,
}

pub async fn create_router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(endpoints::root))
        .routes(routes!(endpoints::get_cities, endpoints::create_city))
        .routes(routes!(endpoints::health_check))
        .with_state(state)
}

pub async fn launch(oapi_router: OpenApiRouter) {
    let (router, oapi) = oapi_router.split_for_parts();
    let router = router.merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", oapi));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
