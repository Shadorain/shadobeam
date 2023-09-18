use anyhow::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

use super::{action::*, message::*, Event, Frame, StatefulList};

pub use base::Base;

use actions::Actions;
use console::Console;
use implants::Implants;
use input::Input;
use output::Output;

use pane::Pane;
mod pane;

mod actions;
mod base;
mod console;
mod implants;
mod input;
mod output;

pub fn center_text(area: Rect, lines: u16) -> Rect {
    Layout::new()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(49),
            Constraint::Length(lines),
            Constraint::Min(0),
        ])
        .split(area)[1]
}

pub trait Component {
    #[allow(unused_variables)]
    fn init(
        &mut self,
        tx: UnboundedSender<Action>,
        message_tx: Option<UnboundedSender<Message>>,
    ) -> Result<()> {
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Option<Action> {
        Some(match event {
            Some(Event::Quit) => Action::Quit,
            Some(Event::AppTick) => Action::Tick,
            Some(Event::RenderTick) => Action::RenderTick,
            Some(Event::Resize(x, y)) => Action::Resize(x, y),
            Some(Event::Key(key_event)) => return self.handle_key_events(key_event),
            Some(Event::Mouse(mouse_event)) => return self.handle_mouse_events(mouse_event),
            Some(_) | None => return None,
        })
    }
    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        None
    }
    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Option<Action> {
        None
    }
    #[allow(unused_variables)]
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        None
    }
    #[allow(unused_variables)]
    fn message(&mut self, message: Message) -> Option<Action> {
        None
    }

    #[allow(unused_variables)]
    fn focus(&mut self, focused: bool) {}

    fn render(&mut self, f: &mut Frame, area: Rect);
}
