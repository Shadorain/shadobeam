use ratatui::{prelude::*, widgets::*};

use super::{Action, Component, Frame};

#[derive(Default)]
pub struct Other {}

impl Component for Other {
    type Action = Action;

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let w = Paragraph::new("HI!").block(
            Block::new()
                .borders(Borders::ALL)
                .title("Other Window")
                .green(),
        );
        f.render_widget(w, area);
    }
}
