use tasks::{beacon_service_client::BeaconServiceClient, ConnectionRequest};
pub mod tasks {
    tonic::include_proto!("tasks");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BeaconServiceClient::connect("http://[::1]:50051").await?;

    let response = client
        .connection(tonic::Request::new(ConnectionRequest { id: 1 }))
        .await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
