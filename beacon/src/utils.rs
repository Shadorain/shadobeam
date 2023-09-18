use std::net::SocketAddr;

use uuid::Uuid;

use crate::common;

#[derive(Debug, Default, Clone)]
pub struct Task {
    pub uuid: Uuid,
    pub shellcode: common::ShellCode,
}

impl From<Task> for common::Task {
    fn from(value: Task) -> Self {
        Self {
            uuid: Some(value.uuid.into()),
            shellcode: Some(value.shellcode),
        }
    }
}

impl From<common::Task> for Task {
    fn from(value: common::Task) -> Self {
        Self {
            uuid: value.uuid.unwrap().into(),
            shellcode: value.shellcode.unwrap(),
        }
    }
}

impl From<common::Uuid> for Uuid {
    fn from(value: common::Uuid) -> Self {
        Self::from_u64_pair(value.high, value.low)
    }
}
impl From<Uuid> for common::Uuid {
    fn from(value: Uuid) -> Self {
        let (high, low) = value.as_u64_pair();
        Self { high, low }
    }
}

#[derive(Debug, Clone)]
pub struct ImplantInfo {
    socket_addr: SocketAddr,
    uuid: Uuid,
}
impl ImplantInfo {
    pub fn new(socket_addr: SocketAddr, uuid: Uuid) -> Self {
        Self { socket_addr, uuid }
    }
}
impl From<common::ImplantInfo> for ImplantInfo {
    fn from(value: common::ImplantInfo) -> Self {
        Self {
            socket_addr: value.ip_address.parse().expect("Invalid socket address."),
            uuid: value.uuid.unwrap().into(),
        }
    }
}
impl From<ImplantInfo> for common::ImplantInfo {
    fn from(value: ImplantInfo) -> Self {
        Self {
            ip_address: value.socket_addr.to_string(),
            uuid: Some(value.uuid.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ImplantControl {
    Add(ImplantInfo),
    Remove(Uuid),
}
impl From<ImplantControl> for crate::iface::implant_info_response::Itype {
    fn from(value: ImplantControl) -> Self {
        match value {
            ImplantControl::Add(info) => Self::Add(info.into()),
            ImplantControl::Remove(uuid) => Self::Remove(uuid.into()),
        }
    }
}
impl From<ImplantInfo> for ImplantControl {
    fn from(value: ImplantInfo) -> Self {
        Self::Add(value)
    }
}
