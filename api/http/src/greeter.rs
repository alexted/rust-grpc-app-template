//! The greeter endpoints: one route per operation, thin like the reference handlers.

use std::sync::Arc;

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use my_service_container::Container;
use my_service_domain::use_cases::{HelloRequest, HelloResponse};

use crate::RouteError;

/// Routes of the greeter service.
pub(crate) fn routes() -> Router<Arc<Container>> {
    Router::new().route("/hello", post(hello))
}

/// Greet the requested name.
async fn hello(
    State(container): State<Arc<Container>>,
    Json(request): Json<HelloRequest>,
) -> Result<Json<HelloResponse>, RouteError> {
    let response = container.hello_use_case.execute(request)?;
    Ok(Json(response))
}
