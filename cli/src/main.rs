//! Process entrypoint: reads the settings, initializes logging, builds the composition root
//! and serves the HTTP and the gRPC adapters over the shared application dependencies.

use std::sync::Arc;

use my_service_api_grpc as grpc;
use my_service_api_http as http;
use my_service_container::Container;
use my_service_infrastructure::logging_config::init_logging;
use my_service_infrastructure::settings::get_config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config();
    init_logging(config);

    log::info!("HTTP server listening on {}", config.http_address());
    log::info!("gRPC server listening on {}", config.grpc_address());

    let container = Arc::new(Container::new());

    tokio::try_join!(
        http::serve(config.http_address(), container.clone(), shutdown_signal()),
        grpc::serve(config.grpc_address(), container.clone(), shutdown_signal()),
    )?;

    Ok(())
}

/// Resolve when the process receives a termination signal, so the servers drain their
/// in-flight requests and release their resources.
async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    log::info!("shutdown signal received, stopping the servers");
}
