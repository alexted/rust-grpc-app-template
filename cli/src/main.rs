#[tokio::main]
async fn main() {
    let mut join_set = tokio::task::JoinSet::new();

    join_set.spawn(my_service_api_http::serve());
    join_set.spawn(my_service_api_grpc::serve());

    while let Some(handle) = join_set.join_next().await {
        handle.expect("Service panic").expect("Service error");
    }
}
