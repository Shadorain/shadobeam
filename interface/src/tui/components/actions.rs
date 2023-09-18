use ratatui::{prelude::*, widgets::*};

use super::{Component, Frame, Pane};

#[derive(Default)]
pub struct Actions {
    focus: bool,
}

impl Component for Actions {
    fn focus(&mut self, focused: bool) {
        self.focus = focused;
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let w = Paragraph::new("TODO: Maybe a list of runnable actions?")
            .block(Pane::Actions.block(self.focus));
        f.render_widget(w, area);
    }
}
