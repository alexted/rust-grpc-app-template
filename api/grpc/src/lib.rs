use std::net::SocketAddr;
use tonic::transport::Server;

mod greeter;

pub async fn serve() -> anyhow::Result<()> {
    // по хорошему вынести в конфиг
    let addr = SocketAddr::from(([0, 0, 0, 0], 50051));

    Server::builder()
        .add_service(greeter::service())
        .serve(addr)
        .await?;

    Ok(())
}
