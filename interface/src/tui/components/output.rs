use ratatui::{prelude::*, widgets::*};

use super::{Action, Component, Frame, Message, Pane, StatefulList};

#[derive(Default)]
pub struct Output {
    output: StatefulList<String>,

    focus: bool,
}

impl Output {
    pub fn push(&mut self, line: String) {
        self.output.push(line)
    }
}

impl Component for Output {
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::ScrollUp => self.output.previous(),
            Action::ScrollDown => self.output.next(),
            Action::ScrollTop => self.output.first(),
            Action::ScrollBottom => self.output.last(),
            _ => (),
        }
        None
    }

    fn message(&mut self, message: Message) -> Option<Action> {
        if let Message::Output(line) = message {
            self.push(line)
        }
        None
    }

    fn focus(&mut self, focused: bool) {
        self.focus = focused;
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let messages: Vec<ListItem> = self
            .output
            .iter()
            .map(|l| ListItem::new(l.to_string()))
            .rev()
            .collect();
        f.render_stateful_widget(
            List::new(messages)
                .highlight_style(Style::new().bold().fg(Color::White))
                .block(Pane::Output.block(self.focus)),
            area,
            &mut self.output.state,
        );
    }
}
