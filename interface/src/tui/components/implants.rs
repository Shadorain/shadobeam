use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use super::{Action, Component, Frame, ImplantsAction, Message, Pane, StatefulList};

#[derive(Default)]
pub struct Implants {
    list: StatefulList<String>,
}

impl Implants {
    pub fn uuid(&self) -> &str {
        self.list.get().expect("An Implant should be selected.")
    }
}

impl Component for Implants {
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        Some(
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => ImplantsAction::NextItem,
                KeyCode::Char('k') | KeyCode::Up => ImplantsAction::PrevItem,
                _ => return None,
            }
            .into(),
        )
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        if let Action::Implants(i) = action {
            match i {
                ImplantsAction::NextItem => self.list.next(),
                ImplantsAction::PrevItem => self.list.previous(),
            }
        }
        None
    }

    fn message(&mut self, message: Message) -> Option<Action> {
        if let Message::Implants(list) = message {
            self.list.replace(list.to_vec())
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let implants: Vec<ListItem> = self
            .list
            .items
            .iter()
            .map(|c| ListItem::new(c.as_str()))
            .collect();
        f.render_stateful_widget(
            List::new(implants)
                .block(Pane::Implants.block().green())
                .highlight_style(Style::new().bold().fg(Color::LightRed))
                .highlight_symbol("❱ "),
            area,
            &mut self.list.state,
        );
    }
}
