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
    pub fn block(&self, focused: bool) -> Block {
        match self {
            Pane::Shadobeam => Block::new()
                .title(ratatui::widgets::block::Title::from(self.as_ref()))
                .title_alignment(Alignment::Center),
            Pane::Input => Block::new()
                .border_style(if focused {
                    Style::new().green()
                } else {
                    Style::new()
                })
                .borders(Borders::ALL),
            _ => Block::new()
                .title(ratatui::widgets::block::Title::from(self.as_ref()))
                .border_style(if focused {
                    Style::new().green()
                } else {
                    Style::new()
                })
                .borders(Borders::ALL),
        }
    }
    pub fn next(self) -> Pane {
        match self {
            Pane::Implants => Pane::Actions,
            Pane::Actions => Pane::Output,
            Pane::Output => Pane::Console,
            Pane::Console => Pane::Input,
            Pane::Input => Pane::Implants,
            _ => unreachable!(),
        }
    }
    pub fn prev(self) -> Pane {
        match self {
            Pane::Implants => Pane::Input,
            Pane::Actions => Pane::Implants,
            Pane::Output => Pane::Actions,
            Pane::Console => Pane::Output,
            Pane::Input => Pane::Console,
            _ => unreachable!(),
        }
    }
}
