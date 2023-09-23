use std::net::{SocketAddr, SocketAddrV4};

use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    None,
    Quit,
    Tick,
    SendTask(Uuid, Task),
    Implants(ImplantControl),
    Output(Uuid, String),
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub uuid: Uuid,
    pub code: (String, Option<Vec<String>>),
}
impl Task {
    pub fn new(uuid: Uuid, cmd: String, arguments: Option<Vec<String>>) -> Self {
        Self {
            uuid,
            code: (cmd, arguments),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImplantControl {
    Add(ImplantInfo),
    Remove(Uuid),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplantInfo {
    pub uuid: Uuid,
    socket_addr: SocketAddr,
}

impl Default for ImplantInfo {
    fn default() -> Self {
        Self {
            uuid: Uuid::nil(),
            socket_addr: SocketAddr::V4(SocketAddrV4::new([0, 0, 0, 0].into(), 0)),
        }
    }
}
impl ImplantInfo {
    pub fn new(socket_addr: SocketAddr, uuid: Uuid) -> Self {
        Self { socket_addr, uuid }
    }
    pub fn socket(&self) -> SocketAddr {
        self.socket_addr
    }
}
