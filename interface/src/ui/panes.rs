use ratatui::{prelude::*, widgets::*};

use strum_macros::AsRefStr;

use super::{App, Frame};

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

    pub fn ui(&self, app: &mut App, f: &mut Frame, area: Rect) {
        match self {
            Panes::Shadobeam | Panes::Actions | Panes::Output => {
                f.render_widget(self.block(), area);
            }
            Panes::Clients => {
                let clients: Vec<ListItem> = app
                    .clients
                    .items
                    .iter()
                    .map(|c| ListItem::new(c.as_str()))
                    .collect();
                f.render_stateful_widget(
                    List::new(clients)
                        .block(self.block())
                        .highlight_style(Style::new().bold().fg(Color::LightRed))
                        .highlight_symbol("❱ "),
                    area,
                    &mut app.clients.state,
                );
            }
            Panes::Console => {
                let console_lines: Vec<ListItem> = app
                    .console
                    .iter()
                    .rev()
                    .map(|line| ListItem::new(line.as_str()))
                    .collect();
                f.render_widget(List::new(console_lines).block(self.block()), area);
            }
        }
    }
}
