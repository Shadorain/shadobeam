use anyhow::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;

use super::{action::*, Event, Frame, Message, StatefulList};

pub use base::Base;
use clients::Clients;
use other::Other;

mod base;
mod clients;
mod other;

pub trait Component {
    type Action: Into<Action>;

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
            Some(Event::Key(key_event)) => {
                return self.handle_key_events(key_event).map(|e| e.into())
            }
            Some(Event::Mouse(mouse_event)) => {
                return self.handle_mouse_events(mouse_event).map(|e| e.into())
            }
            Some(_) | None => return None,
        })
    }
    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Self::Action> {
        None
    }
    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Option<Self::Action> {
        None
    }
    #[allow(unused_variables)]
    fn dispatch(&mut self, action: Self::Action) -> Option<Self::Action> {
        None
    }
    #[allow(unused_variables)]
    fn message(&mut self, message: Message) -> Option<Self::Action> {
        None
    }
    fn render(&mut self, f: &mut Frame, area: Rect);
}
