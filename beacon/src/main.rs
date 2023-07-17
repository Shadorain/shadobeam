use std::collections::{HashMap, VecDeque};
use std::pin::Pin;

use futures_core::Stream;
use tasks::PollResponse;
use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};

use tasks::{
    beacon_service_server::{BeaconService, BeaconServiceServer},
    ConnectionRequest, ConnectionResponse, PollRequest,
};
pub mod tasks {
    tonic::include_proto!("tasks");
}

// #[derive(Hash, PartialEq, Eq, Debug, Default, Copy)]
type ClientID = u32;

type Task = String;

#[derive(Debug, Default)]
pub struct Client {
    id: ClientID,
    tasks: VecDeque<Task>,
}

impl Client {
    pub fn new(id: ClientID) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Beacon {
    clients: RwLock<HashMap<ClientID, Client>>,
}

#[tonic::async_trait]
impl BeaconService for Beacon {
    async fn connection(
        &self,
        request: Request<ConnectionRequest>,
    ) -> Result<Response<ConnectionResponse>, Status> {
        let conn = request.into_inner();
        println!("Got a connection request: {}", conn.id);

        self.clients
            .write()
            .await
            .insert(conn.id, Client::new(conn.id));

        Ok(Response::new(ConnectionResponse {}))
    }

    type PollStream = Pin<Box<dyn Stream<Item = Result<PollResponse, Status>> + Send + 'static>>;

    async fn poll(
        &self,
        request: Request<tonic::Streaming<PollRequest>>,
    ) -> Result<Response<Self::PollStream>, Status> {
        unimplemented!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let beacon = Beacon::default();

    Server::builder()
        .add_service(BeaconServiceServer::new(beacon))
        .serve(addr)
        .await?;

    Ok(())
}
