use std::{
    error::Error,
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use tokio::time;
use tonic::transport::Channel;

use interface::{
    interface_service_client::InterfaceServiceClient, ClientListRequest, ClientListResponse,
    ConnectionRequest,
};
pub mod interface {
    tonic::include_proto!("interface");
}

struct Interface {
    client: InterfaceServiceClient<Channel>,
    uuid: String,

    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Drop for Interface {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)
            .expect("Failed to revert terminal screen");
        self.terminal
            .show_cursor()
            .expect("Failed to display cursor");
    }
}

impl Interface {
    pub async fn connect(url: &'static str) -> Result<Self, Box<dyn Error>> {
        let mut client = InterfaceServiceClient::connect(url).await?;
        let response = client
            .connection(tonic::Request::new(ConnectionRequest {}))
            .await?
            .into_inner();
        println!("Connect: RESPONSE={:?}", response);

        let mut stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");

        Ok(Self {
            client,
            uuid: response.uuid,
            terminal: Terminal::new(CrosstermBackend::new(stdout))?,
        })
    }

    pub async fn get_list(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        let response = self
            .client
            .get_list(tonic::Request::new(ClientListRequest {
                uuid: self.uuid.clone(),
            }))
            .await?
            .into_inner();

        println!("Poll: RESPONSE={:?}", response);

        Ok(response.list)
    }

    fn ui(f: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());

        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);

        let block = Block::default().title("Block 2").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    }

    pub async fn run(mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let list: Vec<ListItem> = self
                .get_list()
                .await?
                .into_iter()
                .map(ListItem::new)
                .collect();
            self.terminal.draw(|frame| {
                let greeting = Paragraph::new("Hello World!");
                frame.render_widget(greeting, frame.size());

                let items = List::new(list);
                frame.render_widget(items, frame.size());
            })?;

            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    if KeyCode::Char('q') == key.code {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Interface::connect("http://[::1]:50055").await?.run().await
}
