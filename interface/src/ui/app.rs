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

    actions: VecDeque<Action>,
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

    fn push_action(&mut self, action: Action) {
        self.actions.push_back(action)
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
                f.render_widget(Panes::Actions.block(), sub_chunks_left[1]);
                f.render_widget(Panes::Output.block(), sub_chunks_right[0]);

                let clients: Vec<ListItem> = self
                    .clients
                    .items
                    .iter()
                    .map(|c| ListItem::new(c.as_str()))
                    .collect();
                f.render_stateful_widget(
                    List::new(clients)
                        .block(Panes::Clients.block())
                        .highlight_style(Style::new().bold().fg(Color::LightRed))
                        .highlight_symbol("❱ "),
                    sub_chunks_left[0],
                    &mut self.clients.state,
                );

                let console_lines: Vec<ListItem> = self
                    .console
                    .iter()
                    .rev()
                    .map(|line| ListItem::new(line.as_str()))
                    .collect();
                f.render_widget(
                    List::new(console_lines).block(Panes::Console.block()),
                    sub_chunks_right[1],
                );

                if let Some(modal) = &self.modal {
                    match modal {
                        Modal::Command(buf) => {
                            let area = Modal::popup_area(50, 10, size);
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
                        KeyCode::Enter => self.clients.next(),
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        Ok(self.pop_action())
    }
}
