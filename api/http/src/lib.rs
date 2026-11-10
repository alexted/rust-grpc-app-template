//! HTTP adapter: the Axum application of the service.
//!
//! The adapter is thin: the routes parse the HTTP requests into the use-case request models,
//! run the use cases taken from the shared [`Container`], and turn the use-case responses and
//! errors into HTTP responses. The greeting business logic lives in the domain crate; nothing
//! in this crate is allowed to know it.

use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use my_service_container::Container;
use my_service_domain::errors::GreeterError;

mod greeter;

/// Build the HTTP application: the greeter routes sharing the application dependencies.
///
/// This is the application factory of the HTTP adapter, like the reference application
/// factory takes the container and publishes its dependencies as the application state.
pub fn create_app(container: Arc<Container>) -> axum::Router {
    axum::Router::new()
        .nest("/greeter", greeter::routes())
        .with_state(container)
}

/// Serve the HTTP application on `addr` until `shutdown` resolves.
pub async fn serve(
    addr: SocketAddr,
    container: Arc<Container>,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, create_app(container).into_make_service())
        .with_graceful_shutdown(shutdown)
        .await?;
    Ok(())
}

/// The HTTP view of an application error.
///
/// The status code and the response body come from the error kind, so the handlers stay free
/// of protocol-specific error handling.
#[derive(Debug)]
pub(crate) struct RouteError {
    inner: GreeterError,
}

impl From<GreeterError> for RouteError {
    fn from(error: GreeterError) -> Self {
        RouteError { inner: error }
    }
}

impl std::fmt::Display for RouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl IntoResponse for RouteError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self.inner {
            GreeterError::InvalidNameLength => StatusCode::BAD_REQUEST,
        };
        (status, self.inner.to_string()).into_response()
    }
}
