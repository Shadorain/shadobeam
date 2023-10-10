use anyhow::Result;
use tonic::transport::Server;

use shadobeam_proto::{
    iface::interface_service_server::InterfaceServiceServer,
    tasks::beacon_service_server::BeaconServiceServer,
};

use beacon::Beacon;

mod beacon;
mod implant;
mod interface;

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
