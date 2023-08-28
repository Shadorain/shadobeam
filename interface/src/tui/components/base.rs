use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::*;
use tokio::sync::mpsc::{self, UnboundedSender};
use uuid::Uuid;

use super::{
    Action, Actions, Component, Console, Frame, Implants, Input, Message, Output, Pane, Task,
};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
enum Mode {
    #[default]
    Normal,
    Insert,
    Processing,
}

#[derive(Default)]
pub struct Base {
    mode: Mode,

    // Panes
    // selected_pane: Pane,

    // Components
    implants: Implants,
    actions: Actions,
    output: Output,
    console: Console,
    input: Input,

    // Channels
    action_tx: Option<mpsc::UnboundedSender<Action>>,
    message_tx: Option<mpsc::UnboundedSender<Message>>,
}

type ComponentList<'a> = [&'a mut dyn Component; 5];

impl<'a> std::ops::Index<Pane> for ComponentList<'a> {
    type Output = &'a mut dyn Component;

    fn index(&self, idx: Pane) -> &Self::Output {
        &self[idx as usize]
    }
}

impl Base {
    pub fn new() -> Self {
        Self::default()
    }

    fn components(&mut self) -> ComponentList {
        [
            &mut self.implants,
            &mut self.actions,
            &mut self.output,
            &mut self.console,
            &mut self.input,
        ]
    }

    fn send(&self, message: Message) {
        if let Some(tx) = &self.message_tx {
            tx.send(message)
                .expect("Base: Send message failure: {message}")
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
    fn layout(area: Rect) -> [Rect; 5] {
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
                Constraint::Min(10),
                Constraint::Min(3),
            ])
            .split(chunks[1]);

        [left[0], left[1], right[0], right[1], right[2]]
    }
}

impl Component for Base {
    fn init(
        &mut self,
        tx: UnboundedSender<Action>,
        message_tx: Option<mpsc::UnboundedSender<Message>>,
    ) -> anyhow::Result<()> {
        self.action_tx = Some(tx.clone());
        self.message_tx = message_tx;

        self.implants.init(tx.clone(), None)?;
        self.actions.init(tx, None)?;
        // self.input.init(tx, None)?;

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        Some(match self.mode {
            Mode::Normal | Mode::Processing => match key.code {
                KeyCode::Char('q') => Action::Quit,
                KeyCode::Char('d') | KeyCode::Char('c')
                    if key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    Action::Quit
                }
                KeyCode::Char('z') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    Action::Suspend
                }
                KeyCode::Char('/') => Action::EnterInsert,

                _ => self.implants.handle_key_events(key).unwrap_or(Action::Tick),
            },
            Mode::Insert => return self.input.handle_key_events(key),
        })
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::CompleteInput => {
                self.send(Message::SendTask(
                    self.implants.uuid().to_string(),
                    Task {
                        uuid: Uuid::new_v4(),
                        code: (self.input.to_string(), None),
                    },
                ));
                self.console.push(self.input.to_string());
                return Some(Action::EnterNormal);
            }
            Action::EnterNormal => {
                self.mode = Mode::Normal;
                self.input.set_insert(false);
            }
            Action::EnterInsert => {
                self.mode = Mode::Insert;
                self.input.set_insert(true);
            }
            Action::EnterProcessing => {
                self.mode = Mode::Processing;
            }
            Action::ExitProcessing => {
                // TODO: Make this go to previous mode instead
                self.mode = Mode::Normal;
            }

            Action::Implants(_) => return self.implants.dispatch(action),

            _ => (),
        }
        None
    }

    fn message(&mut self, message: Message) -> Option<Action> {
        match message {
            Message::Implants(_) => self.implants.message(message),
            Message::Output(_) => self.output.message(message),
            _ => None,
        }
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let layout = Base::layout(area);

        f.render_widget(Pane::Shadobeam.block(), area);
        self.components()
            .iter_mut()
            .enumerate()
            .for_each(|(i, c)| c.render(f, layout[i]));
    }
}
