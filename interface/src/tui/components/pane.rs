use ratatui::{prelude::*, widgets::*};
use strum_macros::AsRefStr;

#[derive(Default, AsRefStr, Clone, Copy)]
pub enum Pane {
    Shadobeam = 99,
    #[default]
    Implants = 0,
    Actions = 1,
    Output = 2,
    Console = 3,
    Input = 4,
}

impl Pane {
    pub fn block(&self) -> Block {
        match self {
            Pane::Shadobeam => Block::new()
                .title(ratatui::widgets::block::Title::from(self.as_ref()))
                .title_alignment(Alignment::Center),
            _ => Block::new()
                .title(ratatui::widgets::block::Title::from(self.as_ref()))
                .borders(Borders::ALL),
        }
    }
}
