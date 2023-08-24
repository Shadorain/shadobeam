use ratatui::{prelude::*, widgets::*};

use super::{Component, Frame, Pane};

#[derive(Default)]
pub struct Actions {}

impl Component for Actions {
    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let w =
            Paragraph::new("TODO: Maybe a list of runnable actions?").block(Pane::Actions.block());
        f.render_widget(w, area);
    }
}
