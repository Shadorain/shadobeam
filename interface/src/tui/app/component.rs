use super::{Action, Frame, KeyEvent, Rect, Result};

pub trait Component {
    fn is_active(&self) -> bool {
        true
    }

    fn draw(&mut self, f: &mut Frame, area: Rect);

    fn event(&mut self, key: KeyEvent) -> Result<Option<Action>>;
}
