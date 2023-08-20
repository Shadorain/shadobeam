#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq)]
pub enum Message {
    None,
    Quit,
    Tick,
    SendTask(String, String),
    Clients(Vec<String>),
}
