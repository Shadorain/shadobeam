use ratatui::{prelude::*, widgets::*};

use super::{Action, Component, Frame, Pane, StatefulList};

#[derive(Default)]
pub struct Console {
    log: StatefulList<String>,
    focus: bool,
}

impl Console {
    pub fn push(&mut self, item: String) {
        self.log.push(item)
    }
}

impl Component for Console {
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::ScrollUp => self.log.previous(),
            Action::ScrollDown => self.log.next(),
            Action::ScrollTop => self.log.first(),
            Action::ScrollBottom => self.log.last(),
            _ => (),
        }
        None
    }

    fn focus(&mut self, focused: bool) {
        self.focus = focused;
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        self.log.render(
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
                    .block(Pane::Console.block(self.focus))
            },
            Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("▲"))
                    .thumb_symbol("█")
                    .track_symbol("│")
                    .end_symbol(Some("▼")),
            ),
        );
    }
}
