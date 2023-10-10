use anyhow::Result;
use clap::Parser;

use interface::Interface;
use tokio::sync::mpsc;
use tui::{utils::*, App, Message};

mod interface;
mod tui;

const APP_TICK_RATE: u64 = 1000;
const RENDER_TICK_RATE: u64 = 50;

// Define the command line arguments structure
#[derive(Parser, Debug)]
#[command(version = version(), about = "Shadobeam Interface")]
struct Cli {
    /// URL to connect to beacon at.
    #[arg(short, long, default_value = "http://[::1]:50055")]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    initialize_logging()?;

    // Fallback catch for panics.
    initialize_panic_handler();

    let (message_tx, message_rx) = mpsc::unbounded_channel::<Message>();
    let (lmessage_tx, lmessage_rx) = mpsc::unbounded_channel::<Message>();

    let mut app = App::new((APP_TICK_RATE, RENDER_TICK_RATE))?;
    tokio::spawn(async move {
        app.run(Some(message_tx), Some(lmessage_rx)).await.unwrap();
    });

    let interface = Interface::connect(cli.url).await?;
    interface.run(lmessage_tx.clone(), message_rx).await
}
