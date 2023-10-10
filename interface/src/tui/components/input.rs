use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tui_input::{backend::crossterm::EventHandler, Input as TuiInput};
use uuid::Uuid;

use super::{Action, Component, Frame, Pane, Task};

#[derive(Clone, Copy, strum::IntoStaticStr)]
enum InputError {
    #[strum(serialize = "Command is empty")]
    CommandEmpty,
}

#[derive(Default)]
pub struct Input {
    input: TuiInput,
    insert: bool,

    focus: bool,
    error: Option<InputError>,
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

    fn validate_input(&self) -> Result<Task, InputError> {
        let input = self.input.to_string();
        let mut input = input.split_whitespace();

        let cmd = input.next().ok_or(InputError::CommandEmpty)?.to_string();
        let args = input.map(|s| s.to_string()).collect::<Vec<String>>();
        Ok(Task::new(
            Uuid::new_v4(),
            cmd,
            if args.is_empty() { None } else { Some(args) },
        ))
    }

    pub fn task(&mut self) -> Option<Task> {
        match self.validate_input() {
            Ok(task) => {
                self.input.reset();
                Some(task)
            }
            Err(e) => {
                self.error = Some(e);
                None
            }
        }
    }
}

impl Component for Input {
    fn focus(&mut self, focused: bool) {
        self.focus = focused;
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        if !self.insert {
            return None;
        }
        Some(match key.code {
            KeyCode::Esc => Action::EnterNormal,
            KeyCode::Enter => Action::CompleteInput,
            _ => {
                self.input.handle_event(&Event::Key(key));
                self.error = None;
                Action::Update
            }
        })
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let width = area.width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let scroll = self.input.visual_scroll(width as usize);
        let title: Line = if let Some(error) = self.error {
            Line::from(vec![
                Span::raw("Enter Input Mode "),
                Span::styled("(ERROR: ", Style::default().fg(Color::Red)),
                Span::styled(
                    Into::<&'static str>::into(error),
                    Style::default().fg(Color::Red),
                ),
                Span::styled(")", Style::default().fg(Color::Red)),
            ])
        } else {
            Line::from(vec![
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
            ])
        };
        let input = Paragraph::new(self.input.value())
            .scroll((0, scroll as u16))
            .block(Pane::Input.block(self.focus).title(title));
        f.render_widget(input, area);

        if self.insert {
            f.set_cursor(
                (area.x + 1 + self.input.cursor() as u16).min(area.x + area.width - 2),
                area.y + 1,
            )
        }
    }
}
