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
    Clients(Vec<String>),
    NextItem,
    PrevItem,
    EnterNormal,
    EnterInsert,
    EnterProcessing,
    ExitProcessing,
    Update,
    Noop,
}
