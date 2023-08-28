use anyhow::Result;
use tonic::transport::Server;

use iface::interface_service_server::InterfaceServiceServer;
use tasks::beacon_service_server::BeaconServiceServer;

use beacon::Beacon;

pub mod common {
    tonic::include_proto!("common");
}
pub mod tasks {
    tonic::include_proto!("tasks");
}
pub mod iface {
    tonic::include_proto!("iface");
}

mod beacon;
mod implant;
mod task;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50055".parse()?;
    let beacon = Beacon::new();

    Ok(Server::builder()
        .add_service(BeaconServiceServer::new(beacon.clone()))
        .add_service(InterfaceServiceServer::new(beacon))
        .serve(addr)
        .await?)
}
