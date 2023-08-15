use std::error::Error;

use interface::Interface;

mod interface;
mod ui;

pub mod common {
    tonic::include_proto!("common");
}
pub mod iface {
    tonic::include_proto!("iface");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Interface::connect("http://[::1]:50055").await?.run().await
}
