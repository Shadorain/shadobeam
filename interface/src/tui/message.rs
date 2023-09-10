use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    None,
    Quit,
    Tick,
    SendTask(Uuid, Task),
    Implants(Vec<String>),
    Output(String),
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub uuid: Uuid,
    pub code: (String, Option<Vec<String>>),
}
