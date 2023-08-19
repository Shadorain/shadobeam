use std::{collections::VecDeque, io::Stdout};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};

use modal::Modal;
use panes::Panes;
use stateful_list::StatefulList;

use super::Result;

pub use component::Component;

mod component;
mod modal;
mod panes;
mod stateful_list;

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
type Uuid = String;

pub enum Action {
    SendTask(Uuid, String),
}

#[derive(Default)]
pub enum State {
    #[default]
    Main,
}

pub struct App {
    state: State,
    quit: bool,
    modal: Modal,

    clients: StatefulList<String>,
    output: String,
    console: Vec<String>,

    actions: VecDeque<Action>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::Main,
            quit: false,
            modal: Modal::new(),

            clients: StatefulList::new(),
            output: String::new(),
            console: Vec::new(),

            actions: VecDeque::new(),
        }
    }
    pub fn should_quit(&self) -> bool {
        self.quit
    }
    pub fn update_clients(&mut self, clients: Vec<String>) {
        self.clients = StatefulList::with_items(clients);
        self.clients.next();
    }
    pub fn pop_action(&mut self) -> Option<Action> {
        self.actions.pop_front()
    }

    pub(super) fn push_action(&mut self, action: Action) {
        self.actions.push_back(action);
        // self.console.push(action);
    }

    pub(super) fn uuid(&self) -> &String {
        self.clients.get().expect("UUID should be focused.")
    }

    /// Generic Next Method
    /// Based on current selected pane.
    fn next(&mut self) {
        self.clients.next();
    }
    /// Generic Previous Method
    /// Based on current selected pane.
    fn prev(&mut self) {
        self.clients.previous();
    }

    /// Generates the standard layout using passed in area.
    ///
    /// Return values:
    /// - [0]: Clients
    /// - [1]: Actions
    /// - [2]: Output
    /// - [3]: Console
    ///
    /// * `area`: Frame size to use.
    fn layout(area: Rect) -> (Rect, Rect, Rect, Rect) {
        let chunks = Layout::new()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .margin(1)
            .split(area);

        let left = Layout::new()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);
        let right = Layout::new()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(chunks[1]);

        (left[0], left[1], right[0], right[1])
    }
}

impl Component for App {
    fn draw(&mut self, f: &mut Frame, area: Rect) {
        match self.state {
            State::Main => {
                let layout = App::layout(area);
                Panes::Shadobeam.ui(self, f, area);
                Panes::Clients.ui(self, f, layout.0);
                Panes::Actions.ui(self, f, layout.1);
                Panes::Output.ui(self, f, layout.2);
                Panes::Console.ui(self, f, layout.3);

                if self.modal.is_active() {
                    self.modal.draw(f, area);
                }
            }
        }
    }

    fn event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if self.modal.is_active() {
            self.modal.event(key)?;
        } else {
            match key.code {
                KeyCode::Char('q') /* if key.modifiers == KeyModifiers::SHIFT */ => {
                    self.quit = true
                }
                KeyCode::Char('j') | KeyCode::Down => self.next(),
                KeyCode::Char('k') | KeyCode::Up => self.prev(),

                KeyCode::Char('a') => self.modal.create('a'),
                _ => (),
            }
        }
        Ok(self.pop_action())
    }
}
