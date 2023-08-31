use ratatui::{prelude::*, widgets::*};

use super::{Action, Component, Frame, Message, Pane, StatefulList};

#[derive(Default)]
pub struct Implants {
    list: StatefulList<String>,
    focus: bool,
}

impl Implants {
    pub fn uuid(&self) -> &str {
        self.list.get().expect("An Implant should be selected.")
    }
}

impl Component for Implants {
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::ScrollUp => self.list.previous(),
            Action::ScrollDown => self.list.next(),
            Action::ScrollTop => self.list.first(),
            Action::ScrollBottom => self.list.last(),
            _ => (),
        }
        None
    }

    fn message(&mut self, message: Message) -> Option<Action> {
        if let Message::Implants(list) = message {
            self.list.replace(list.to_vec());
        }
        None
    }

    fn focus(&mut self, focused: bool) {
        self.focus = focused;
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        self.list.render(
            f,
            area,
            |items| {
                let list: Vec<ListItem> = items.iter().map(|c| ListItem::new(c.as_str())).collect();
                List::new(list)
                    .block(Pane::Implants.block(self.focus))
                    .highlight_style(Style::new().bold().fg(Color::LightRed))
                    .highlight_symbol("❱ ")
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
