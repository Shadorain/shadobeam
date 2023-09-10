use ratatui::{prelude::*, widgets::*};
use uuid::Uuid;

use super::{center_text, Action, Component, Frame, Message, Pane, StatefulList};

#[derive(Default)]
struct Implant {
    id_str: String,
    id: Uuid,
}
impl Implant {
    pub fn new(id_str: String) -> Self {
        let id = Uuid::parse_str(&id_str).expect("Failed to parse implant uuid.");
        Implant { id_str, id }
    }
}

#[derive(Default)]
pub struct Implants {
    list_map: StatefulList<Implant>,
    focus: bool,
}

impl Implants {
    pub fn uuid(&self) -> Option<Uuid> {
        self.list_map.get().map(|x| x.id)
    }
}

impl Component for Implants {
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::ScrollUp => self.list_map.previous(),
            Action::ScrollDown => self.list_map.next(),
            Action::ScrollTop => self.list_map.first(),
            Action::ScrollBottom => self.list_map.last(),
            _ => (),
        }
        None
    }

    fn message(&mut self, message: Message) -> Option<Action> {
        if let Message::Implants(list) = message {
            self.list_map
                .replace(list.into_iter().map(|id| Implant::new(id)).collect());
        }
        if self.list_map.changed() {
            return Some(Action::ImplantChanged);
        }
        None
    }

    fn focus(&mut self, focused: bool) {
        self.focus = focused;
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let focus = self.focus;
        if self.list_map.len() > 0 {
            self.list_map.render(
                f,
                area,
                |items| {
                    let list: Vec<ListItem> = items
                        .iter()
                        .map(|c| ListItem::new(c.id_str.as_str()))
                        .collect();
                    List::new(list)
                        .block(Pane::Implants.block(focus))
                        .highlight_style(Style::new().bold().fg(Color::LightRed))
                        .highlight_symbol("❱ ")
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
                Paragraph::new("No implants found.").alignment(Alignment::Center),
                center_text(area, 1),
            );
            f.render_widget(Pane::Implants.block(focus), area)
        }
    }
}
