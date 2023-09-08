use std::collections::HashMap;

use ratatui::{prelude::*, widgets::*};

use super::{center_text, Action, Component, Frame, Pane, StatefulList};

#[derive(Default)]
pub struct Console {
    log: HashMap<String, StatefulList<String>>,
    current_key: Option<String>,

    focus: bool,
}

impl Console {
    pub fn push(&mut self, item: String) {
        if let Some(list) = self.current() {
            list.push(item)
        }
    }
    pub fn current(&mut self) -> Option<&mut StatefulList<String>> {
        let key = self.current_key.as_ref()?;
        self.log.get_mut(key)
    }

    pub fn set_key(&mut self, key: String) -> Option<usize> {
        self.log.insert(key.clone(), StatefulList::new());

        self.current_key = Some(key);
        if let Some(list) = self.current() {
            return list.state.selected();
        }
        None
    }
}

impl Component for Console {
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        let list = self.current()?;
        match action {
            Action::ScrollUp => list.previous(),
            Action::ScrollDown => list.next(),
            Action::ScrollTop => list.first(),
            Action::ScrollBottom => list.last(),
            _ => (),
        }
        if let Some(list) = self.current() {
            if list.changed() {
                return Some(Action::ConsoleChanged(list.state.selected()?));
            }
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
                        .map(|(i, m)| {
                            ListItem::new(vec![Line::from(Span::raw(format!(
                                "{}): (cmd) ❱ {}",
                                i, m
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
