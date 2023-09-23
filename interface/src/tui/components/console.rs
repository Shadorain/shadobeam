use std::collections::HashMap;

use ratatui::{prelude::*, widgets::*};
use uuid::Uuid;

use super::{center_text, Action, Component, Frame, ImplantControl, Pane, StatefulList, Task};

type Key = Uuid;
type SList = StatefulList<Task>;

#[derive(Default)]
pub struct Console {
    list_map: HashMap<Key, SList>,
    current_key: Option<Key>,

    focus: bool,
}

impl Console {
    pub fn push(&mut self, task: Task) {
        if let Some(list) = self.current() {
            list.push(task);
        }
    }
    pub fn current(&mut self) -> Option<&mut SList> {
        let key = self.current_key.as_ref()?;
        self.list_map.get_mut(key)
    }

    pub fn implant_control(&mut self, control: &ImplantControl) {
        match control {
            ImplantControl::Add(info) => {
                self.list_map.insert(info.uuid, StatefulList::new());
                self.current_key = Some(info.uuid);
                self.current().unwrap().first();
            }
            ImplantControl::Remove(uuid) => {
                self.list_map.remove(uuid);
                self.current_key = Some(*uuid);
            }
        };
    }

    pub fn set_key(&mut self, key: Option<Key>) -> Option<Uuid> {
        self.current_key = key;
        if let Some(list) = self.current() {
            return Some(list.get()?.uuid);
        }
        None
    }
}

impl Component for Console {
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        let list = self.current()?;
        if let Action::List(m) = action {
            list.movement(m);
            return Some(Action::ConsoleChanged(list.get().map(|task| task.uuid)));
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
                        .enumerate()
                        .map(|(i, task)| {
                            ListItem::new(vec![Line::from(Span::raw(format!(
                                "{}): (cmd) ❱ {}",
                                i, task.code.0
                            )))])
                        })
                        .rev()
                        .collect();
                    List::new(list)
                        .highlight_style(Style::new().bold().fg(Color::White))
                        .block(Pane::Console.block(focus))
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
                Paragraph::new("No previous commands found.").alignment(Alignment::Center),
                center_text(area, 1),
            );
            f.render_widget(Pane::Console.block(focus), area)
        }
    }
}
