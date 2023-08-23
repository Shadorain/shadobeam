use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};
use strum_macros::AsRefStr;
use tokio::sync::mpsc::{self, UnboundedSender};
use tui_input::{backend::crossterm::EventHandler, Input};

use super::{Action, Clients, Component, Frame, Message, Other};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
enum Mode {
    #[default]
    Normal,
    Insert,
    Processing,
}

#[derive(Default, AsRefStr)]
pub enum Pane {
    Shadobeam,
    #[default]
    Clients,
    Actions,
    Output,
    Console,
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

#[derive(Default)]
pub struct Base {
    clients: Clients,
    input: Input,
    mode: Mode,

    selected_pane: Pane,

    other: Other,
    show_other: bool,

    action_tx: Option<mpsc::UnboundedSender<Action>>,
    message_tx: Option<mpsc::UnboundedSender<Message>>,
}

impl Base {
    pub fn new() -> Self {
        Self::default()
    }

    fn send(&self, message: Message) {
        if let Some(tx) = &self.message_tx {
            tx.send(message)
                .expect("Base: Send message failure: {message}")
        }
    }

    fn render_input(&mut self, f: &mut Frame, area: Rect) {
        let width = area.width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let scroll = self.input.visual_scroll(width as usize);
        let input = Paragraph::new(self.input.value())
            .style(match self.mode {
                Mode::Insert => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
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

        if self.mode == Mode::Insert {
            f.set_cursor(
                (area.x + 1 + self.input.cursor() as u16).min(area.x + area.width - 2),
                area.y + 1,
            )
        }
    }

    /// Generates the standard layout using passed in area.
    ///
    /// Return values:
    /// - [0]: Clients
    /// - [1]: Actions
    /// - [2]: Output
    /// - [3]: Console
    /// - [4]: Input
    ///
    /// * `area`: Frame size to use.
    fn layout(area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        let chunks = Layout::new()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .margin(1)
            .split(area);

        let left = Layout::new()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(30)])
            .split(chunks[0]);
        let right = Layout::new()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(75),
                Constraint::Percentage(25),
                Constraint::Min(3),
            ])
            .split(chunks[1]);

        (left[0], left[1], right[0], right[1], right[2])
    }
}

impl Component for Base {
    type Action = Action;

    fn init(
        &mut self,
        tx: UnboundedSender<Action>,
        message_tx: Option<mpsc::UnboundedSender<Message>>,
    ) -> anyhow::Result<()> {
        self.action_tx = Some(tx.clone());
        self.message_tx = message_tx;

        self.other.init(tx.clone(), None)?;
        self.clients.init(tx, None)?;

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Self::Action> {
        Some(match self.mode {
            Mode::Normal | Mode::Processing => match key.code {
                KeyCode::Char('q') => Action::Quit,
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
                KeyCode::Char('z') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Action::Suspend
                }
                KeyCode::Char('l') => Action::ToggleShowLogger,
                KeyCode::Char('/') => Action::EnterInsert,

                _ => {
                    if let Some(a) = self.clients.handle_key_events(key) {
                        a.into()
                    } else {
                        Action::Tick
                    }
                }
            },
            Mode::Insert => match key.code {
                KeyCode::Esc => Action::EnterNormal,
                KeyCode::Enter => Action::CompleteInput(self.input.to_string()),
                _ => {
                    self.input.handle_event(&Event::Key(key));
                    Action::Update
                }
            },
        })
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::ToggleShowLogger => self.show_other = !self.show_other,
            Action::EnterNormal => {
                self.mode = Mode::Normal;
            }
            Action::CompleteInput(t) => {
                self.send(Message::SendTask(self.clients.uuid().to_string(), t));
                return Some(Action::EnterNormal);
            }
            Action::EnterInsert => {
                self.mode = Mode::Insert;
            }
            Action::EnterProcessing => {
                self.mode = Mode::Processing;
            }
            Action::ExitProcessing => {
                // TODO: Make this go to previous mode instead
                self.mode = Mode::Normal;
            }

            Action::Clients(c) => return self.clients.dispatch(c).map(|i| i.into()),

            _ => (),
        }
        None
    }

    fn message(&mut self, message: Message) -> Option<Self::Action> {
        if let Message::Clients(_) = message {
            return self.clients.message(message).map(|i| i.into());
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let layout = Base::layout(area);

        f.render_widget(Pane::Shadobeam.block(), area);
        self.clients.render(f, layout.0);
        f.render_widget(Pane::Actions.block(), layout.1);
        f.render_widget(Pane::Output.block(), layout.2);
        f.render_widget(Pane::Console.block(), layout.3);
        self.render_input(f, layout.4);
    }
}
