use std::collections::HashMap;

use ratatui::{prelude::*, widgets::*};

use super::{center_text, Action, Component, Frame, Message, Pane, StatefulList};

#[derive(Default)]
pub struct Output {
    output: HashMap<String, HashMap<usize, StatefulList<String>>>,
    current_key: Option<(String, usize)>,

    focus: bool,
}

impl Output {
    pub fn current(&mut self) -> Option<&mut StatefulList<String>> {
        let key = self.current_key.as_ref()?;
        self.output.get_mut(&key.0)?.get_mut(&key.1)
    }

    pub fn set_key(&mut self, key: (String, usize)) {
        self.output
            .insert(key.0.clone(), HashMap::from([(key.1, StatefulList::new())]));
        self.current_key = Some(key);
    }
}

impl Component for Output {
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        let list = self.current()?;
        match action {
            Action::ScrollUp => list.previous(),
            Action::ScrollDown => list.next(),
            Action::ScrollTop => list.first(),
            Action::ScrollBottom => list.last(),
            _ => (),
        }
        None
    }

    fn message(&mut self, message: Message) -> Option<Action> {
        if let Message::Output(line) = message {
            self.current()?.push(line)
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
                    let list: Vec<ListItem> =
                        items.iter().map(|c| ListItem::new(c.as_str())).collect();
                    List::new(list)
                        .highlight_style(Style::new().bold().fg(Color::White))
                        .block(Pane::Output.block(focus))
                },
                Some(
                    Scrollbar::new(ScrollbarOrientation::VerticalRight)
                        .begin_symbol(Some("▲"))
                        .thumb_symbol("█")
                        .track_symbol("│")
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
