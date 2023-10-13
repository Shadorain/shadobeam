use shadobeam_proto::{ImplantControl, OutputResult, Task};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    None,
    Quit,
    Tick,
    SendTask(Uuid, Task),
    Implants(ImplantControl),
    Output((Uuid, Uuid), OutputResult),
}
