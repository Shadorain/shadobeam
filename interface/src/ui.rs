use std::{
    error::Error,
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

pub use app::App;
use panes::Panes;

mod app;
mod modal;
mod panes;
mod stateful_list;

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;
type Uuid = String;

pub enum Action {
    SendTask(Uuid, String),
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Drop for UI {
    fn drop(&mut self) {
        self.cleanup()
    }
}

impl UI {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");

        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout))?,
        })
    }
    pub fn cleanup(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)
            .expect("Failed to revert terminal screen");

        self.terminal
            .show_cursor()
            .expect("Failed to display cursor");
    }

    pub fn events(
        &mut self,
        app: &mut App,
        poll_rate: u64,
    ) -> Result<Option<Action>, Box<dyn Error>> {
        // let widget = match item {
        //     Drawable::ClientList(list) => List::new(
        //         list.into_iter()
        //             .map(ListItem::new)
        //             .collect::<Vec<ListItem>>(),
        //     ),
        // };

        self.terminal.draw(|f| app.ui(f))?;

        if event::poll(Duration::from_millis(poll_rate))? {
            return app.event(event::read()?);
        }

        Ok(None)
    }
}
