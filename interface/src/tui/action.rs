use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Resume,
    Suspend,

    Tick,
    RenderTick,
    Resize(u16, u16),

    CompleteInput,
    ImplantChanged,
    ConsoleChanged(Option<Uuid>),

    NextPane,
    PrevPane,

    List(Movement),

    EnterNormal,
    EnterInsert,
    EnterProcessing,
    ExitProcessing,

    Update,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    Up,
    Down,
    ScrollTop,
    ScrollBottom,
}
