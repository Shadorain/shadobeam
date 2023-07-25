use tasks::{beacon_service_client::BeaconServiceClient, ConnectionRequest, PollRequest};
pub mod tasks {
    tonic::include_proto!("tasks");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BeaconServiceClient::connect("http://[::1]:50051").await?;

    let response = client
        .connection(tonic::Request::new(ConnectionRequest { }))
        .await?;
    println!("Connect: RESPONSE={:?}", response);

    let response = client
        .poll(tonic::Request::new(PollRequest { uuid: response.into_inner().uuid }))
        .await?;

    println!("Poll: RESPONSE={:?}", response);

    Ok(())
}
