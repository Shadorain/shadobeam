use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};
use tokio::{sync::mpsc::{self, UnboundedReceiver, UnboundedSender}, task::JoinHandle};
use tui_input::{backend::crossterm::EventHandler, Input};

use super::{Action, Clients, Component, Frame, Message, Other};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
enum Mode {
    #[default]
    Normal,
    Insert,
    Processing,
}

#[derive(Default)]
pub struct Base {
    clients: Clients,
    input: Input,
    mode: Mode,

    other: Other,
    show_other: bool,

    action_tx: Option<mpsc::UnboundedSender<Action>>,
    message_tx: Option<UnboundedSender<Message>>,
    message_rx: Option<UnboundedReceiver<Message>>,

    task: Option<JoinHandle<()>>,
}

impl Base {
    pub fn new() -> Self {
        Self::default()
    }

    fn uuid(&self) -> String {
        String::from("some-uuid")
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
            .constraints([Constraint::Percentage(75), Constraint::Percentage(20), Constraint::Percentage(5)])
            .split(chunks[1]);

        (left[0], left[1], right[0], right[1], right[2])
    }
}

impl Component for Base {
    type Action = Action;

    fn init(
        &mut self,
        tx: UnboundedSender<Action>,
        message_tx: Option<UnboundedSender<Message>>,
        message_rx: Option<UnboundedReceiver<Message>>,
    ) -> anyhow::Result<()> {
        self.action_tx = Some(tx.clone());
        self.message_tx = message_tx.clone();
        if message_rx.is_some() {
            let tx = tx.clone();
            let task = tokio::spawn(async move {
                #[allow(clippy::unnecessary_unwrap)]
                let mut rx = message_rx.unwrap();
                loop {
                    if let Some(message) = &rx.recv().await {
                        match message {
                            Message::Clients(list) => tx.send(super::ClientsAction::List(list.to_vec()).into()).unwrap(),
                            Message::None => (),
                            _ => (),
                        }
                    }
                }
            });
            self.task = Some(task);
        }

        self.other.init(tx, message_tx, None)?;
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
                _ => Action::Tick,
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
                if let Some(tx) = &self.message_tx {
                    tx.send(Message::SendTask(self.uuid(), t)).unwrap();
                }
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
            _ => (),
        }
        None
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let layout = Base::layout(area);

        f.render_widget(
            Block::new()
                .title("Shadobeam")
                .title_alignment(Alignment::Center),
            area,
        );

        self.clients.render(f, layout.0);

        let width = layout.1.width.max(3) - 3; // keep 2 for borders and 1 for cursor
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
        f.render_widget(input, layout.4);

        if self.mode == Mode::Insert {
            f.set_cursor(
                (layout.1.x + 1 + self.input.cursor() as u16).min(layout.1.x + layout.1.width - 2),
                layout.1.y + 1,
            )
        }
    }
}
