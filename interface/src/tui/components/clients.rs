use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

use super::{ClientsAction, Component, Frame, StatefulList};

#[derive(Default)]
pub struct Clients {
    list: StatefulList<String>,
}

impl Component for Clients {
    type Action = ClientsAction;

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Self::Action> {
        Some(match key.code {
            KeyCode::Char('j') | KeyCode::Down => ClientsAction::NextItem,
            KeyCode::Char('k') | KeyCode::Up => ClientsAction::PrevItem,
            _ => return None,
        })
    }

    fn dispatch(&mut self, action: Self::Action) -> Option<Self::Action> {
        match action {
            ClientsAction::List(list) => self.list.replace(list.to_vec()),
            ClientsAction::NextItem => self.list.next(),
            ClientsAction::PrevItem => self.list.previous(),
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let clients: Vec<ListItem> = self
            .list
            .items
            .iter()
            .map(|c| ListItem::new(c.as_str()))
            .collect();
        f.render_stateful_widget(
            List::new(clients)
                .block(Block::new().title("Clients").borders(Borders::ALL))
                .highlight_style(Style::new().bold().fg(Color::LightRed))
                .highlight_symbol("❱ "),
            area,
            &mut self.list.state,
        );
    }
}
