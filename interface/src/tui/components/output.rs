use std::collections::HashMap;

use ratatui::{prelude::*, widgets::*};
use shadobeam_proto::OutputResult;
use uuid::Uuid;

use super::{center_text, Action, Component, Frame, Message, Pane, StatefulList};

type Key = (Uuid, Uuid);
type SList = StatefulList<OutputResult>;

#[derive(Default)]
pub struct Output {
    output_map: HashMap<Key, SList>,
    current_key: Option<Key>,

    focus: bool,
}

impl Output {
    pub fn current(&mut self) -> Option<&mut SList> {
        let key = self.current_key.as_ref()?;
        self.output_map.get_mut(key)
    }
    pub fn current_with(&mut self, key: Key) -> Option<&mut SList> {
        self.output_map.get_mut(&key)
    }

    pub fn add_console(&mut self, key: Key) {
        self.output_map.insert(key, StatefulList::new());
        self.set_key(Some(key));
    }
    pub fn remove_implant(&mut self, uuid: Uuid) {
        self.output_map.retain(|k, _| k.0 != uuid);
    }

    pub fn set_key(&mut self, key: Option<Key>) {
        self.current_key = key;
    }
}

impl Component for Output {
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        let list = self.current()?;
        if let Action::List(m) = action {
            list.movement(m);
        }
        None
    }

    fn message(&mut self, message: Message) -> Option<Action> {
        if let Message::Output(key, line) = message {
            self.current_with(key)?.push(line)
        }
        None
    }

    fn focus(&mut self, focused: bool) {
        self.focus = focused;
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let focus = self.focus;
        if let Some(list) = self.current() {
            list.render(
                f,
                area,
                |items| {
                    let list: Vec<ListItem> = items
                        .iter()
                        .map(|res| match res {
                            Ok(line) => ListItem::new(line.as_str()),
                            Err(e) => {
                                ListItem::new(e.as_str()).style(Style::new().bold().fg(Color::Red))
                            }
                        })
                        .collect();
                    List::new(list)
                        .highlight_style(Style::new().bold().fg(Color::White))
                        .block(Pane::Output.block(focus))
                },
                Some(
                    Scrollbar::new(ScrollbarOrientation::VerticalRight)
                        .begin_symbol(Some("▲"))
                        .thumb_symbol("█")
                        .track_symbol(Some("│"))
                        .end_symbol(Some("▼")),
                ),
            );
        } else {
            f.render_widget(
                Paragraph::new("No implants or commands selected.").alignment(Alignment::Center),
                center_text(area, 1),
            );
            f.render_widget(Pane::Output.block(focus), area)
        }
    }
}
