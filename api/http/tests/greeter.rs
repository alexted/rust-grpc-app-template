//! API tests of the greeter endpoints: the router is exercised in memory with `tower`'s
//! `oneshot`, no listening socket is needed.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use my_service_api_http::create_app;
use my_service_container::Container;
use tower::ServiceExt;

fn app() -> axum::Router {
    create_app(Arc::new(Container::new()))
}

#[tokio::test]
async fn hello_greets_the_requested_name() {
    let response = app()
        .oneshot(
            Request::post("/greeter/hello")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"Alice"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["message"], "Hello, Alice!");
}

#[tokio::test]
async fn hello_rejects_a_name_shorter_than_three_bytes() {
    let response = app()
        .oneshot(
            Request::post("/greeter/hello")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"ab"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(body.as_ref(), b"Invalid name length");
}
