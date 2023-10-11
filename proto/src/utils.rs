use std::net::{SocketAddr, SocketAddrV4};

use uuid::Uuid;

use crate::{common, iface::implant_info_response::Itype};

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

impl From<Task> for common::Task {
    fn from(value: Task) -> Self {
        Self {
            uuid: Some(value.uuid.into()),
            shellcode: Some(common::ShellCode {
                command: value.code.0,
                arguments: value.code.1.unwrap_or_default(),
            }),
        }
    }
}

impl From<common::Task> for Task {
    fn from(value: common::Task) -> Self {
        let code = value.shellcode.unwrap();
        Self {
            uuid: value.uuid.unwrap().into(),
            code: (
                code.command,
                if code.arguments.is_empty() {
                    None
                } else {
                    Some(code.arguments)
                },
            ),
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

impl From<common::ImplantInfo> for ImplantInfo {
    fn from(value: common::ImplantInfo) -> Self {
        Self::new(
            value.ip_address.parse().expect("Invalid socket address."),
            value.uuid.unwrap().into(),
        )
    }
}
impl From<ImplantInfo> for common::ImplantInfo {
    fn from(value: ImplantInfo) -> Self {
        Self {
            uuid: Some(value.uuid.into()),
            ip_address: value.socket().to_string(),
        }
    }
}

impl From<ImplantControl> for Itype {
    fn from(value: ImplantControl) -> Self {
        match value {
            ImplantControl::Add(info) => Self::Add(info.into()),
            ImplantControl::Remove(uuid) => Self::Remove(uuid.into()),
        }
    }
}
impl From<Itype> for ImplantControl {
    fn from(value: Itype) -> Self {
        match value {
            Itype::Add(info) => Self::Add(info.into()),
            Itype::Remove(uuid) => Self::Remove(uuid.into()),
        }
    }
}
impl From<ImplantInfo> for ImplantControl {
    fn from(value: ImplantInfo) -> Self {
        Self::Add(value)
    }
}

pub type OutputResult = Result<String, String>;

impl From<OutputResult> for common::OutputResult {
    fn from(value: OutputResult) -> Self {
        match value {
            Ok(line) => Self {
                result: Some(common::output_result::Result::Line(line)),
            },
            Err(err) => Self {
                result: Some(common::output_result::Result::Error(err)),
            },
        }
    }
}
impl From<common::OutputResult> for OutputResult {
    fn from(value: common::OutputResult) -> Self {
        if let Some(res) = value.result {
            match res {
                common::output_result::Result::Line(line) => Self::Ok(line),
                common::output_result::Result::Error(err) => Self::Err(err),
            }
        } else {
            Self::Err("Output result is None".to_string())
        }
    }
}
