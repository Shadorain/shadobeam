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
            uuid: value.uuid.to_string(),
            shellcode: Some(value.shellcode),
        }
    }
}

impl From<common::Task> for Task {
    fn from(value: common::Task) -> Self {
        Self {
            uuid: Uuid::parse_str(&value.uuid).expect("Failed to parse task UUID."),
            shellcode: value.shellcode.unwrap(),
        }
    }
}
