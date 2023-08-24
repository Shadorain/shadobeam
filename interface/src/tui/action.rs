#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    Resume,
    Suspend,
    Tick,
    RenderTick,
    Resize(u16, u16),
    CompleteInput,
    EnterNormal,
    EnterInsert,
    EnterProcessing,
    ExitProcessing,
    Update,
    Implants(ImplantsAction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImplantsAction {
    NextItem,
    PrevItem,
}

impl From<ImplantsAction> for Action {
    fn from(value: ImplantsAction) -> Self {
        Self::Implants(value)
    }
}
