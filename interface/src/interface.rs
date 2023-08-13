use crate::{
    iface::{
        interface_service_client::InterfaceServiceClient, ClientListRequest, ConnectionRequest,
    },
    ui::{App, Drawable, UI},
};

use std::error::Error;

use tonic::transport::Channel;

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

    pub async fn run(mut self) -> Result<(), Box<dyn Error>> {
        let mut app = App::new();
        loop {
            // let list = Drawable::ClientList(self.get_list().await?);
            self.ui.events(&mut app, 250)?;

            if app.should_quit() {
                break;
            }
        }
        Ok(())
    }
}
