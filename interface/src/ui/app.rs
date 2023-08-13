use std::error::Error;

use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use strum_macros::AsRefStr;

use super::{Frame, Panes};

#[derive(Default)]
pub enum State {
    #[default]
    Main,
}

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
                        app.console = buf.to_string();
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

#[derive(Default)]
pub struct App {
    state: State,
    modal: Option<Modal>,
    quit: bool,

    clients: Vec<String>,
    output: String,
    console: String,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub(super) fn ui(&mut self, f: &mut Frame) {
        let size = f.size();
        match self.state {
            State::Main => {
                let chunks = Layout::new()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                    .margin(1)
                    .split(size);

                let sub_chunks_left = Layout::new()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(chunks[0]);
                let sub_chunks_right = Layout::new()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                    .split(chunks[1]);

                f.render_widget(Panes::Shadobeam.block(), size);
                f.render_widget(Panes::Clients.block(), sub_chunks_left[0]);
                f.render_widget(Panes::Actions.block(), sub_chunks_left[1]);
                f.render_widget(Panes::Output.block(), sub_chunks_right[0]);

                f.render_widget(
                    Paragraph::new(self.console.as_str()).block(Panes::Console.block()),
                    sub_chunks_right[1],
                );

                if let Some(modal) = &self.modal {
                    match modal {
                        Modal::Command(buf) => {
                            let area = Modal::popup_area(60, 20, size);
                            f.render_widget(Clear, area); //this clears out the background
                            f.render_widget(
                                Paragraph::new(buf.as_str()).block(modal.block()),
                                area,
                            );
                        }
                    }
                }
            }
        }
    }

    pub(super) fn event(&mut self, event: Event) -> Result<(), Box<dyn Error>> {
        #[allow(clippy::single_match)]
        match event {
            Event::Key(key) => {
                if self.modal.is_some() {
                    Modal::key_event(key, self);
                } else {
                    match key.code {
                        KeyCode::Char('q') => self.quit = true,
                        KeyCode::Char('a') => self.modal = Some(Modal::Command(String::new())),
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        Ok(())
    }
}
