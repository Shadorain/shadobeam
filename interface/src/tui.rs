use std::{
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

use app::Component;
pub use app::{Action, App};
mod app;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Drop for Tui {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)
            .expect("Failed to revert terminal screen");

        self.terminal
            .show_cursor()
            .expect("Failed to display cursor");
    }
}

impl Tui {
    pub fn new() -> Result<Self> {
        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");

        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout))?,
        })
    }

    pub fn events(&mut self, app: &mut App, poll_rate: u64) -> Result<Option<Action>> {
        self.terminal.draw(|f| app.draw(f, f.size()))?;

        if event::poll(Duration::from_millis(poll_rate))? {
            #[allow(clippy::single_match)]
            match event::read()? {
                Event::Key(key) => {
                    return app.event(key);
                }
                _ => (),
            }
        }

        Ok(None)
    }
}
