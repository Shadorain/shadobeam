use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tui_input::{backend::crossterm::EventHandler, Input as TuiInput};

use super::{Action, Component, Frame};

#[derive(Default)]
pub struct Input {
    input: TuiInput,
    insert: bool,
}

impl ToString for Input {
    fn to_string(&self) -> String {
        self.input.to_string()
    }
}

impl Input {
    pub fn set_insert(&mut self, is_insert: bool) {
        self.insert = is_insert;
    }
}

impl Component for Input {
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        if !self.insert {
            return None;
        }
        Some(match key.code {
            KeyCode::Esc => Action::EnterNormal,
            KeyCode::Enter => Action::CompleteInput,
            _ => {
                self.input.handle_event(&Event::Key(key));
                Action::Update
            }
        })
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let width = area.width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let scroll = self.input.visual_scroll(width as usize);
        let input = Paragraph::new(self.input.value())
            .scroll((0, scroll as u16))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Line::from(vec![
                        Span::raw("Enter Input Mode "),
                        Span::styled("(Press ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            "/",
                            Style::default()
                                .add_modifier(Modifier::BOLD)
                                .fg(Color::Gray),
                        ),
                        Span::styled(" to start, ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            "ESC",
                            Style::default()
                                .add_modifier(Modifier::BOLD)
                                .fg(Color::Gray),
                        ),
                        Span::styled(" to finish)", Style::default().fg(Color::DarkGray)),
                    ])),
            );
        f.render_widget(input, area);

        if self.insert {
            f.set_cursor(
                (area.x + 1 + self.input.cursor() as u16).min(area.x + area.width - 2),
                area.y + 1,
            )
        }
    }
}
