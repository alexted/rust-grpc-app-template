use my_service_domain::greeter as domain;
use proto::{
    greeter_server::{self, GreeterServer},
    HelloReply, HelloRequest,
};
use tonic::{Request, Response, Status};

mod proto {
    tonic::include_proto!("greeter");
}

pub(crate) fn service() -> GreeterServer<Greeter> {
    GreeterServer::new(Greeter)
}

#[derive(Debug)]
pub(crate) struct Greeter;

impl From<HelloRequest> for domain::HelloInput {
    fn from(input: HelloRequest) -> Self {
        Self { name: input.name }
    }
}

impl From<domain::HelloOutput> for HelloReply {
    fn from(output: domain::HelloOutput) -> Self {
        Self {
            message: output.message,
        }
    }
}

#[tonic::async_trait]
impl greeter_server::Greeter for Greeter {
    async fn hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        let request = request.into_inner();
        let reply = domain::hello(request.into()).map_err(|e| match e {
            e @ domain::GreeterError::InvalidNameLength => {
                Status::new(tonic::Code::InvalidArgument, e.to_string())
            }
        })?;

        Ok(Response::new(reply.into()))
    }
}
