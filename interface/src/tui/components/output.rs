use std::collections::HashMap;

use ratatui::{prelude::*, widgets::*};
use shadobeam_proto::OutputResult;
use uuid::Uuid;

use super::{center_text, Action, Component, Frame, Message, Pane, StatefulList};

type Key = (Uuid, Uuid);
type SList = StatefulList<OutputResult>;

#[derive(Default)]
pub struct Output {
    output_map: HashMap<Key, SList>,
    current_key: Option<Key>,

    focus: bool,
    // throbber_state: throbber_widgets_tui::ThrobberState,
}

impl Output {
    pub fn current(&mut self) -> Option<&mut SList> {
        let key = self.current_key.as_ref()?;
        self.output_map.get_mut(key)
    }
    pub fn current_with(&mut self, key: Key) -> Option<&mut SList> {
        self.output_map.get_mut(&key)
    }

    pub fn add_console(&mut self, key: Key) {
        self.output_map.insert(key, StatefulList::new());
        self.set_key(Some(key));
    }
    pub fn remove_implant(&mut self, uuid: Uuid) {
        self.output_map.retain(|k, _| k.0 != uuid);
    }

    pub fn set_key(&mut self, key: Option<Key>) {
        self.current_key = key;
    }
}

impl Component for Output {
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        let list = self.current()?;
        if let Action::List(m) = action {
            list.movement(m);
        }
        None
    }

    fn message(&mut self, message: Message) -> Option<Action> {
        if let Message::Output(key, line) = message {
            self.current_with(key)?.push(line)
        }
        None
    }

    fn focus(&mut self, focused: bool) {
        self.focus = focused;
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let focus = self.focus;
        let mut center_pane = |par: Paragraph| {
            f.render_widget(par.alignment(Alignment::Center), center_text(area, 1));
            f.render_widget(Pane::Output.block(focus), area);
        };
        // self.throbber_state.calc_next();
        if let Some(list) = self.current() {
            if !list.is_empty() {
                list.render(
                    f,
                    area,
                    |items| {
                        let list: Vec<ListItem> = items
                            .iter()
                            .map(|res| match res {
                                Ok(line) => ListItem::new(line.as_str()),
                                Err(e) => ListItem::new(e.as_str())
                                    .style(Style::new().bold().fg(Color::Red)),
                            })
                            .collect();
                        List::new(list)
                            .highlight_style(Style::new().bold().fg(Color::White))
                            .block(Pane::Output.block(focus))
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
                center_pane(
                    Paragraph::new("Waiting on implant...").style(Style::new().light_magenta()),
                );
                // let throbber = throbber_widgets_tui::Throbber::default()
                //     .label("Waiting on implant... ")
                //     .style(Style::new().light_magenta())
                //     .throbber_style(Style::new().cyan())
                //     .throbber_set(throbber_widgets_tui::BRAILLE_DOUBLE)
                //     .use_type(throbber_widgets_tui::WhichUse::Spin);
                // f.render_stateful_widget(throbber, center_text(area, 1), &mut self.throbber_state);
                // f.render_widget(Pane::Output.block(focus), area);
            }
        } else {
            center_pane(Paragraph::new("No implants or commands selected."));
        }
    }
}
