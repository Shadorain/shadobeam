use anyhow::Result;

use super::{
    common::ShellCode,
    iface::{
        interface_service_client::InterfaceServiceClient, AddTaskRequest, ClientListRequest,
        ConnectionRequest,
    },
};

use tonic::transport::Channel;

type Task = ShellCode;

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

    pub async fn add_task(&mut self, client_uuid: String, task: String) -> Result<()> {
        self.client
            .add_task(tonic::Request::new(AddTaskRequest {
                uuid: self.uuid.clone(),
                client_uuid,
                task: Some(Task {
                    command: task,
                    arguments: None,
                }),
            }))
            .await?;

        Ok(())
    }
}
