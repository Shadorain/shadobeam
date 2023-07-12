use tasks::{greeter_server::GreeterServer, HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};

use crate::tasks::greeter_server::Greeter;

pub mod tasks {
    tonic::include_proto!("tasks");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        Ok(Response::new(tasks::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
