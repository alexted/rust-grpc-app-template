use crate::{RouteError, RouteResult};
use axum::{http::StatusCode, routing::post, Json};
use my_service_domain::greeter as domain;
use serde::{Deserialize, Serialize};

pub(crate) fn routes() -> axum::Router {
    axum::Router::new().route("/hello", post(hello))
}

#[derive(Debug, Deserialize)]
struct HelloRequest {
    name: String,
}

impl From<HelloRequest> for domain::HelloInput {
    fn from(input: HelloRequest) -> Self {
        Self { name: input.name }
    }
}

#[derive(Debug, Serialize)]
struct HelloResponse {
    message: String,
}

impl From<domain::HelloOutput> for HelloResponse {
    fn from(output: domain::HelloOutput) -> Self {
        Self {
            message: output.message,
        }
    }
}

async fn hello(Json(request): Json<HelloRequest>) -> RouteResult<Json<HelloResponse>> {
    let response = domain::hello(request.into()).map_err(|e| match e {
        e @ domain::GreeterError::InvalidNameLength => {
            RouteError::new(e).with_status(StatusCode::BAD_REQUEST)
        }
    })?;
    Ok(Json(response.into()))
}
