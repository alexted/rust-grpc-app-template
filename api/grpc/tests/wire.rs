//! Wire tests of the greeter service: a generated client talks to the real server over a TCP
//! socket, proving the RPC contract, the response payloads and the status mapping on the wire.

use std::sync::Arc;

use my_service_api_grpc::greeter_service;
use my_service_container::Container;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

// Same generated-code lint allowances as in the crate itself (see src/greeter.rs).
#[allow(clippy::result_large_err, clippy::double_must_use)]
mod proto {
    tonic::include_proto!("greeter");
}

use proto::greeter_client::GreeterClient;
use proto::HelloRequest;

/// Serve the greeter service on an OS-assigned port of the loopback interface and return the
/// bound address.
async fn spawn_server() -> std::net::SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        Server::builder()
            .add_service(greeter_service(Arc::new(Container::new())))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .unwrap();
    });

    addr
}

#[tokio::test]
async fn hello_greets_the_requested_name_over_the_wire() {
    let addr = spawn_server().await;
    let mut client = GreeterClient::connect(format!("http://{addr}"))
        .await
        .unwrap();

    let reply = client
        .hello(HelloRequest {
            name: "Alice".into(),
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(reply.message, "Hello, Alice!");
}

#[tokio::test]
async fn hello_maps_short_names_to_invalid_argument_over_the_wire() {
    let addr = spawn_server().await;
    let mut client = GreeterClient::connect(format!("http://{addr}"))
        .await
        .unwrap();

    let status = client
        .hello(HelloRequest { name: "ab".into() })
        .await
        .unwrap_err();

    assert_eq!(status.code(), tonic::Code::InvalidArgument);
    assert_eq!(status.message(), "Invalid name length");
}
