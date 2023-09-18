use uuid::Uuid;

use crate::{
    common,
    iface::implant_info_response::Itype,
    tui::{ImplantControl, ImplantInfo, Task},
};

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
