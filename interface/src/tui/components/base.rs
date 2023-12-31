use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use ratatui::prelude::*;
use tokio::sync::mpsc::{self, UnboundedSender};

use super::{
    Action, Actions, Component, Console, Frame, Implants, Input, Message, Movement, Output, Pane,
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
    selected_pane: (Pane, Pane),

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

    fn index(&self, index: Pane) -> &Self::Output {
        &self[index as usize]
    }
}
impl<'a> std::ops::IndexMut<Pane> for ComponentList<'a> {
    fn index_mut(&mut self, index: Pane) -> &mut Self::Output {
        &mut self[index as usize]
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

    fn revert_pane(&mut self) {
        self.select_pane(self.selected_pane.1);
    }
    fn select_pane(&mut self, pane: Pane) {
        let mut cmp = self.components();
        cmp.iter_mut().for_each(|c| c.focus(false));

        cmp[pane].focus(true);
        self.selected_pane.1 = self.selected_pane.0;
        self.selected_pane.0 = pane;
    }
    fn next_pane(&mut self) {
        self.select_pane(self.selected_pane.0.next());
    }
    fn prev_pane(&mut self) {
        self.select_pane(self.selected_pane.0.prev());
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
        self.select_pane(Pane::Implants);

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

                KeyCode::Up | KeyCode::Char('k') => Action::List(Movement::Up),
                KeyCode::Down | KeyCode::Char('j') => Action::List(Movement::Down),
                KeyCode::Home | KeyCode::Char('g') => Action::List(Movement::ScrollTop),
                KeyCode::End | KeyCode::Char('G') => Action::List(Movement::ScrollBottom),

                KeyCode::Tab | KeyCode::Char('l') | KeyCode::Right => Action::NextPane,
                KeyCode::BackTab | KeyCode::Char('h') | KeyCode::Left => Action::PrevPane,

                _ => {
                    let pane = self.selected_pane.0;
                    let cmp = &mut self.components()[pane];
                    cmp.handle_key_events(key).unwrap_or(Action::Tick)
                }
            },
            Mode::Insert => return self.input.handle_key_events(key),
        })
    }

    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Option<Action> {
        Some(match self.mode {
            Mode::Normal | Mode::Processing => match mouse.kind {
                MouseEventKind::ScrollUp => Action::List(Movement::Up),
                MouseEventKind::ScrollDown => Action::List(Movement::Down),
                _ => return None,
            },
            _ => return None,
        })
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::CompleteInput => {
                // TODO: Handle if uuid is None.
                if let Some(uuid) = self.implants.uuid() {
                    let task = self.input.task()?;
                    self.send(Message::SendTask(uuid, task.clone()));
                    self.output.add_console((uuid, task.uuid));
                    self.console.push(task);
                }
                return Some(Action::EnterNormal);
            }

            Action::NextPane => self.next_pane(),
            Action::PrevPane => self.prev_pane(),

            Action::EnterNormal => {
                self.mode = Mode::Normal;
                self.input.set_insert(false);
            }
            Action::EnterInsert => {
                self.select_pane(Pane::Input);
                self.mode = Mode::Insert;
                self.input.set_insert(true);
            }
            Action::EnterProcessing => {
                self.mode = Mode::Processing;
            }
            Action::ExitProcessing => {
                // TODO: Make this go to previous mode instead
                self.mode = Mode::Normal;
                self.revert_pane();
            }

            Action::ImplantChanged => {
                return Some(Action::ConsoleChanged(
                    self.console.set_key(self.implants.uuid()),
                ));
            }
            Action::ConsoleChanged(k) => {
                self.output
                    .set_key(if let Some(uuid) = self.implants.uuid() {
                        k.map(|k| (uuid, k))
                    } else {
                        None
                    });
            }

            _ => {
                let pane = self.selected_pane.0;
                let cmp = &mut self.components()[pane];
                return cmp.dispatch(action);
            }
        }
        None
    }

    fn message(&mut self, message: Message) -> Option<Action> {
        match message {
            Message::Implants(ref control) => {
                self.console.implant_control(control);
                if let super::ImplantControl::Remove(uuid) = control {
                    self.output.remove_implant(*uuid);
                }
                self.implants.message(message)
            }
            Message::Output(..) => self.output.message(message),
            _ => None,
        }
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let layout = Base::layout(area);

        f.render_widget(Pane::Shadobeam.block(false), area);
        self.components()
            .iter_mut()
            .enumerate()
            .for_each(|(i, c)| c.render(f, layout[i]));
    }
}
