use crate::{
    iface::{
        interface_service_client::InterfaceServiceClient, ClientListRequest, ConnectionRequest, AddTaskRequest,
    },
    ui::{App, UI, Action}, common::ShellCode,
};

use std::error::Error;

use tonic::transport::Channel;

type Task = ShellCode;

pub struct Interface {
    client: InterfaceServiceClient<Channel>,
    uuid: String,

    ui: UI,
}

impl Interface {
    pub async fn connect(url: &'static str) -> Result<Self, Box<dyn Error>> {
        let mut client = InterfaceServiceClient::connect(url).await?;
        let response = client
            .connection(tonic::Request::new(ConnectionRequest {}))
            .await?
            .into_inner();

        // println!("Connect: RESPONSE={:?}", response);

        Ok(Self {
            client,
            uuid: response.uuid,
            ui: UI::new()?,
        })
    }

    async fn get_list(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        let response = self
            .client
            .get_list(tonic::Request::new(ClientListRequest {
                uuid: self.uuid.clone(),
            }))
            .await?
            .into_inner();

        // println!("Poll: RESPONSE={:?}", response);

        Ok(response.list)
    }

    async fn add_task(&mut self, client_uuid: String, task: String) -> Result<(), Box<dyn Error>> {
        let _ = self
            .client
            .add_task(tonic::Request::new(AddTaskRequest {
                uuid: self.uuid.clone(),
                client_uuid,
                task: Some(Task { command: task, arguments: None })
            }))
            .await?
            .into_inner();

        Ok(())
    }

    pub async fn run(mut self) -> Result<(), Box<dyn Error>> {
        let mut app = App::new();
        app.update_clients(self.get_list().await?);

        loop {
            // let list = Drawable::ClientList(self.get_list().await?);
            if let Some(action) = self.ui.events(&mut app, 250)? {
                match action {
                    Action::SendTask(u, t) => self.add_task(u, t).await?,
                }
            }

            if app.should_quit() {
                break;
            }
        }
        Ok(())
    }
}
