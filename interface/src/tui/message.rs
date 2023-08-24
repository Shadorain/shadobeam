#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    None,
    Quit,
    Tick,
    SendTask(String, String),
    Implants(Vec<String>),
}
