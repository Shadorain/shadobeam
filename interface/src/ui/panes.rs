use ratatui::{
    prelude::Alignment,
    widgets::{Block, Borders},
};

use strum_macros::AsRefStr;

#[derive(AsRefStr)]
pub enum Panes {
    Shadobeam,
    Clients,
    Actions,
    Output,
    Console,
}

impl Panes {
    pub fn block(&self) -> Block {
        match self {
            Panes::Shadobeam => Block::new()
                .title(ratatui::widgets::block::Title::from(self.as_ref()))
                .title_alignment(Alignment::Center),
            _ => Block::new()
                .title(ratatui::widgets::block::Title::from(self.as_ref()))
                .borders(Borders::ALL),
        }
    }
}
