use ratatui::{prelude::*, widgets::*};

use super::{Component, Frame, Pane, StatefulList};

#[derive(Default)]
pub struct Console {
    log: StatefulList<String>,
}

impl Console {
    pub fn push(&mut self, item: String) {
        self.log.push(item)
    }
}

impl Component for Console {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        let messages: Vec<ListItem> = self
            .log
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
        f.render_stateful_widget(
            List::new(messages).block(Pane::Console.block()),
            area,
            &mut self.log.state,
        );
    }
}
