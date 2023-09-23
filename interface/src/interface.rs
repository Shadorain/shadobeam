use anyhow::Result;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

use super::{
    iface::{
        interface_service_client::InterfaceServiceClient, AddTaskRequest, ConnectionRequest,
        ImplantInfoRequest,
    },
    Message, Task,
};

use tonic::transport::Channel;

pub struct Interface {
    client: InterfaceServiceClient<Channel>,
    uuid: Uuid,
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
            uuid: response.uuid.unwrap().into(),
        })
    }

    pub async fn run(
        mut self,
        msg_tx: UnboundedSender<Message>,
        mut msg_rx: UnboundedReceiver<Message>,
    ) -> Result<()> {
        self.implant_info(msg_tx.clone()).await?;

        while let Some(message) = msg_rx.recv().await {
            match message {
                Message::SendTask(c_id, task) => self.add_task(c_id, task, msg_tx.clone()).await?,
                Message::Quit => break,
                _ => (),
            }
        }
        Ok(())
    }

    async fn implant_info(&mut self, tx: UnboundedSender<Message>) -> Result<()> {
        let mut response = self
            .client
            .implant_info(tonic::Request::new(ImplantInfoRequest {
                uuid: Some(self.uuid.into()),
            }))
            .await?
            .into_inner();

        tokio::spawn(async move {
            while let Some(r) = response.message().await.unwrap() {
                tx.send(Message::Implants(r.itype.unwrap().into())).unwrap();
            }
        });
        Ok(())
    }

    async fn add_task(
        &mut self,
        client_uuid: Uuid,
        task: Task,
        tx: UnboundedSender<Message>,
    ) -> Result<()> {
        log::info!("Sending task: {task:?}");

        let uuid = task.uuid;
        let mut response = self
            .client
            .add_task(tonic::Request::new(AddTaskRequest {
                uuid: Some(self.uuid.into()),
                client_uuid: Some(client_uuid.into()),
                task: Some(task.into()),
            }))
            .await?
            .into_inner();

        tokio::spawn(async move {
            while let Some(r) = response.message().await.unwrap() {
                if let Some(line) = r.line {
                    tx.send(Message::Output(uuid, line)).unwrap();
                } else {
                    break;
                }
            }
        });
        Ok(())
    }
}
