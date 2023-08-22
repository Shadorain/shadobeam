#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    Resume,
    Suspend,
    Tick,
    RenderTick,
    Resize(u16, u16),
    ToggleShowLogger,
    CompleteInput(String),
    EnterNormal,
    EnterInsert,
    EnterProcessing,
    ExitProcessing,
    Update,
    None,
    Clients(ClientsAction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientsAction {
    List(Vec<String>),
    NextItem,
    PrevItem,
}

impl From<ClientsAction> for Action {
    fn from(value: ClientsAction) -> Self {
        Self::Clients(value)
    }
}
