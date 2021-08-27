use tonic::transport::Server;

mod protos;

use crate::protos::api::{HelloReply, HelloRequest};
use thiserror::Error;
use tonic::{Request, Response, Status};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("unknown server error")]
    Unknown,

    #[error("tonic transport error")]
    TonicError(#[from] tonic::transport::Error),
}

struct Greeter;

#[tonic::async_trait]
impl protos::api::greeter_server::Greeter for Greeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply {
            message: format!("hello {}", request.into_inner().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let server = protos::api::greeter_server::GreeterServer::new(Greeter);

    let addr = "[::1]:50051".parse().unwrap();

    Server::builder()
        .accept_http1(true)
        .add_service(server)
        .serve(addr)
        .await?;

    Ok(())
}
