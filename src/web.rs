use crate::endpoints;
use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use sqlx::Pool;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<sqlx::Postgres>,
}

pub async fn create_basic_router(state: AppState) -> Router {
    create_oapi_router(state).await.split_for_parts().0
}

pub async fn create_full_router(state: AppState) -> Router {
    // Add OpenAPI routes
    let (router, oapi) = create_oapi_router(state).await.split_for_parts();
    let router = router.merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", oapi));

    // Add Prometheus route
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    router
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
}

async fn create_oapi_router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(endpoints::root))
        .routes(routes!(endpoints::get_cities, endpoints::create_city))
        .routes(routes!(endpoints::health_check))
        .with_state(state)
}

pub async fn launch(router: Router, bind_addr: String) {
    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
