use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use strum_macros::AsRefStr;

use super::{Action, Component, Frame, Result};

#[derive(AsRefStr)]
pub enum ModalType {
    Command(String),
}
impl ModalType {
    fn block(&self) -> Block {
        match self {
            ModalType::Command(_) => Block::new()
                .title(ratatui::widgets::block::Title::from(self.as_ref()))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        }
    }
}

pub struct Modal {
    modal: Option<ModalType>,
}

impl Modal {
    pub fn new() -> Self {
        Self { modal: None }
    }

    pub fn create(&mut self, code: char) {
        match code {
            'a' => self.modal = Some(ModalType::Command(String::new())),
            _ => (),
        }
    }

    fn area(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

impl Component for Modal {
    fn is_active(&self) -> bool {
        self.modal.is_some()
    }

    fn draw(&mut self, f: &mut Frame, area: Rect) {
        if self.is_active() {
            let modal = self.modal.as_ref().unwrap();
            match modal {
                ModalType::Command(buf) => {
                    let area = Modal::area(50, 10, area);
                    f.render_widget(Clear, area);
                    f.render_widget(Paragraph::new(buf.as_str()).block(modal.block()), area);
                }
            }
        }
    }

    fn event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if self.is_active() {
            match self.modal.as_mut().unwrap() {
                ModalType::Command(buf) => match key.code {
                    KeyCode::Char(k) => buf.push(k),
                    KeyCode::Backspace => {
                        buf.pop();
                    }
                    KeyCode::Enter => {
                        let buf = buf.to_string();
                        self.modal = None;
                        return Ok(Some(Action::SendTask(String::new(), buf)));
                    }
                    KeyCode::Esc => self.modal = None,
                    _ => (),
                },
            }
        }
        Ok(None)
    }
}
