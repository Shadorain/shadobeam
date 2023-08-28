use ratatui::{prelude::*, widgets::*};

use super::{Action, Component, Frame, Message, Pane, StatefulList};

#[derive(Default)]
pub struct Output {
    output: StatefulList<String>,
}

impl Output {
    pub fn push(&mut self, line: String) {
        self.output.push(line)
    }
}

impl Component for Output {
    fn message(&mut self, message: Message) -> Option<Action> {
        if let Message::Output(line) = message {
            self.push(line)
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let messages: Vec<ListItem> = self
            .output
            .iter()
            .map(|l| ListItem::new(l.to_string()))
            .rev()
            .collect();
        f.render_stateful_widget(
            List::new(messages).block(Pane::Console.block()),
            area,
            &mut self.output.state,
        );
    }
}
