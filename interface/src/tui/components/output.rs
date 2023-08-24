use ratatui::{prelude::*, widgets::*};

use super::{Component, Frame, Pane};

#[derive(Default)]
pub struct Output {}

impl Component for Output {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        let w = Paragraph::new("TODO: Output of run tasks").block(Pane::Output.block());
        f.render_widget(w, area);
    }
}
