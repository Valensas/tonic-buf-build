use std::result::Result;

use proto::{hello_service_server::HelloServiceServer, SayHelloResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod proto {
    tonic::include_proto!("tonic_buf_build_sample");
}

#[derive(Clone)]
struct HelloServer {}

#[tonic::async_trait]
impl proto::hello_service_server::HelloService for HelloServer {
    async fn say_hello(&self, _: Request<()>) -> Result<Response<SayHelloResponse>, Status> {
        Ok(Response::new(SayHelloResponse {
            value: "Hello world!".into(),
        }))
    }
}

#[tokio::main]
async fn main() {
    let hello_server = HelloServer {};

    Server::builder()
        .add_service(HelloServiceServer::new(hello_server))
        .serve("127.0.0.1:10000".parse().unwrap())
        .await
        .unwrap();
}
