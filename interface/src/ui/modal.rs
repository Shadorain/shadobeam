use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use strum_macros::AsRefStr;

use super::{App, Frame};

#[derive(AsRefStr)]
pub enum Modal {
    Command(String),
}

impl Modal {
    pub fn key_event(key: KeyEvent, app: &mut App) {
        if let Some(m) = &mut app.modal {
            match m {
                Modal::Command(buf) => match key.code {
                    KeyCode::Char(k) => buf.push(k),
                    KeyCode::Backspace => {
                        buf.pop();
                    }
                    KeyCode::Enter => {
                        app.console.push(buf.to_string());
                        app.actions.push_back(super::Action::SendTask(
                            app.clients
                                .get()
                                .expect("UUID should be focused.")
                                .to_string(),
                            buf.to_string(),
                        ));
                        app.modal = None;
                    }
                    KeyCode::Esc => app.modal = None,
                    _ => (),
                },
            }
        }
    }
    pub fn block(&self) -> Block {
        match self {
            Modal::Command(_) => Block::new()
                .title(ratatui::widgets::block::Title::from(self.as_ref()))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        }
    }

    pub fn ui(&self, f: &mut Frame) {
        match self {
            Modal::Command(buf) => {
                let area = Modal::popup_area(50, 10, f.size());
                f.render_widget(Clear, area);
                f.render_widget(Paragraph::new(buf.as_str()).block(self.block()), area);
            }
        }
    }

    pub fn popup_area(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_y) / 2),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage((100 - percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }
}
