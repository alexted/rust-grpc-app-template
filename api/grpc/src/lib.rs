//! gRPC adapter: the Tonic server of the service.
//!
//! The adapter is thin: the services convert the protobuf messages into the use-case request
//! models, run the use cases taken from the shared [`Container`], and turn the use-case
//! responses and errors into gRPC responses and statuses. The generated protobuf code stays
//! inside this crate; the greeting business logic lives in the domain crate.

use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use my_service_container::Container;
use tonic::transport::Server;

mod greeter;

pub use greeter::service as greeter_service;
pub use greeter::GreeterService;

/// Serve the gRPC application on `addr` until `shutdown` resolves.
pub async fn serve(
    addr: SocketAddr,
    container: Arc<Container>,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> anyhow::Result<()> {
    Server::builder()
        .add_service(greeter::service(container))
        .serve_with_shutdown(addr, shutdown)
        .await?;
    Ok(())
}
