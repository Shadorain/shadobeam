use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use tokio::sync::{mpsc::UnboundedSender, RwLock};
use uuid::Uuid;

use crate::utils::ImplantControl;

type HM = HashMap<Uuid, Interface>;

#[derive(Debug, Default)]
pub struct Interfaces {
    interfaces: Arc<RwLock<HM>>,
}

impl Interfaces {
    pub async fn add(&self, uuid: Uuid, interface: Interface) {
        let mut map = self.interfaces.write().await;
        map.insert(uuid, interface);
    }
    pub async fn set_channel(&self, uuid: Uuid, channel: UnboundedSender<ImplantControl>) -> Result<()> {
        let mut map = self.interfaces.write().await;
        map.get_mut(&uuid).ok_or(anyhow!("Interface does not exist."))?.set_channel(channel);
        Ok(())
    }

    pub async fn implant_control(&self, control: ImplantControl) {
        let map = self.interfaces.read().await;

        for (_, val) in map.iter() {
            val.send_control(control.clone()).await;
        }
    }
}

#[derive(Debug, Default)]
pub struct Interface {
    info_channel: Option<RwLock<UnboundedSender<ImplantControl>>>,
}
impl Interface {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_channel(&mut self, channel: UnboundedSender<ImplantControl>) {
        self.info_channel = Some(RwLock::new(channel));
    }
    pub async fn send_control(&self, control: ImplantControl) {
        if let Some(ch) = &self.info_channel {
            ch.write().await.send(control).unwrap();
        }
    }
}
