use anyhow::Result;
use ratatui::{prelude::*, widgets::*};
use uuid::Uuid;

use super::{
    center_text, Action, Component, Frame, ImplantControl, ImplantInfo, Message, Pane, StatefulList,
};

#[derive(Default)]
struct Implant {
    info: ImplantInfo,
}
impl Implant {
    pub fn new(info: ImplantInfo) -> Self {
        Implant { info }
    }
}

#[derive(Default)]
pub struct Implants {
    list: StatefulList<Implant>,
    focus: bool,
}

impl Implants {
    pub fn uuid(&self) -> Option<Uuid> {
        self.list.get().map(|x| x.info.uuid)
    }
}

impl Component for Implants {
    fn init(
        &mut self,
        _: tokio::sync::mpsc::UnboundedSender<Action>,
        _: Option<tokio::sync::mpsc::UnboundedSender<Message>>,
    ) -> Result<()> {
        self.list.first();
        Ok(())
    }
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        if let Action::List(m) = action {
            self.list.movement(m);
            return Some(Action::ImplantChanged);
        }
        None
    }

    fn message(&mut self, message: Message) -> Option<Action> {
        if let Message::Implants(control) = message {
            match control {
                ImplantControl::Add(info) => self.list.push(Implant::new(info)),
                ImplantControl::Remove(uuid) => self.list.retain(|val| val.info.uuid != uuid),
            }
            return Some(Action::ImplantChanged);
        }
        None
    }

    fn focus(&mut self, focused: bool) {
        self.focus = focused;
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let focus = self.focus;
        if self.list.len() > 0 {
            self.list.render(
                f,
                area,
                |items| {
                    let list: Vec<ListItem> = items
                        .iter()
                        .map(|c| ListItem::new(c.info.uuid.to_string()))
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
