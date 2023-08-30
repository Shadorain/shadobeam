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

    NextPane,
    PrevPane,

    ScrollUp,
    ScrollDown,
    ScrollTop,
    ScrollBottom,

    EnterNormal,
    EnterInsert,
    EnterProcessing,
    ExitProcessing,

    Update,
}
