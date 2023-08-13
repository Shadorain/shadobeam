use std::error::Error;

pub mod iface {
    tonic::include_proto!("interface");
}

use interface::Interface;

mod interface;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Interface::connect("http://[::1]:50055").await?.run().await
}
