use anyhow::Result;
use clap::Parser;

use interface::Interface;
use tokio::sync::mpsc;
use tui::{initialize_panic_handler, version, App, Message};

mod interface;
mod tui;

pub mod common {
    tonic::include_proto!("common");
}
pub mod iface {
    tonic::include_proto!("iface");
}

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
    // Fallback catch for panics.
    initialize_panic_handler();

    let cli = Cli::parse();
    let mut interface = Interface::connect(cli.url).await?;
    let (message_tx, mut message_rx) = mpsc::unbounded_channel::<Message>();
    let (lmessage_tx, lmessage_rx) = mpsc::unbounded_channel::<Message>();

    let mut app = App::new((APP_TICK_RATE, RENDER_TICK_RATE))?;
    tokio::spawn(async move {
        app.run(Some(message_tx), Some(lmessage_rx)).await.unwrap();
    });

    loop {
        if let Some(message) = message_rx.recv().await {
            match message {
                Message::SendTask(u, t) => interface.add_task(u, t).await?,
                Message::Tick => lmessage_tx.send(Message::Implants(interface.get_list().await?))?,
                Message::Quit => break,
                _ => (),
            }
        }
    }
    Ok(())
}
