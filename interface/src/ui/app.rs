use std::{collections::VecDeque, error::Error};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{prelude::*, widgets::*};

use super::{modal::Modal, stateful_list::StatefulList, Action, Frame, Panes};

#[derive(Default)]
pub enum State {
    #[default]
    Main,
}

pub struct App {
    state: State,
    quit: bool,
    pub(super) modal: Option<Modal>,

    pub(super) clients: StatefulList<String>,
    pub(super) output: String,
    pub(super) console: Vec<String>,

    pub(super) actions: VecDeque<Action>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::Main,
            quit: false,
            modal: None,

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
        self.actions.push_back(action)
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

                Panes::Shadobeam.ui(self, f, size);
                Panes::Actions.ui(self, f, sub_chunks_left[1]);
                Panes::Output.ui(self, f, sub_chunks_right[0]);
                Panes::Clients.ui(self, f, sub_chunks_left[0]);
                Panes::Console.ui(self, f, sub_chunks_right[1]);

                if let Some(modal) = &self.modal {
                    modal.ui(f);
                }
            }
        }
    }

    pub(super) fn event(&mut self, event: Event) -> Result<Option<Action>, Box<dyn Error>> {
        #[allow(clippy::single_match)]
        match event {
            Event::Key(key) => {
                if self.modal.is_some() {
                    Modal::key_event(key, self);
                } else {
                    match key.code {
                        KeyCode::Char('q') /* if key.modifiers == KeyModifiers::SHIFT */ => {
                            self.quit = true
                        }
                        KeyCode::Char('a') => self.modal = Some(Modal::Command(String::new())),
                        KeyCode::Char('j') | KeyCode::Down => self.next(),
                        KeyCode::Char('k') | KeyCode::Up => self.prev(),
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        Ok(self.pop_action())
    }
}
