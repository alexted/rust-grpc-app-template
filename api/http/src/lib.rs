use axum::{http::StatusCode, response::IntoResponse};
use std::{fmt, net::SocketAddr};
use tokio::net::TcpListener;

mod greeter;

pub async fn serve() -> anyhow::Result<()> {
    // по хорошему вынести в конфиг
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let app = axum::Router::new().nest("/greeter", greeter::routes());

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[derive(Debug)]
struct RouteError {
    status: Option<StatusCode>,
    inner: anyhow::Error,
}

type RouteResult<T> = Result<T, RouteError>;

impl RouteError {
    fn new<M>(message: M) -> Self
    where
        M: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        Self {
            status: None,
            inner: anyhow::Error::msg(message),
        }
    }

    fn with_status(self, status: StatusCode) -> Self {
        Self {
            status: Some(status),
            inner: self.inner,
        }
    }
}

impl<T: Into<anyhow::Error>> From<T> for RouteError {
    fn from(error: T) -> Self {
        Self {
            status: None,
            inner: error.into(),
        }
    }
}

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl IntoResponse for RouteError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, self.inner.to_string()).into_response()
    }
}
