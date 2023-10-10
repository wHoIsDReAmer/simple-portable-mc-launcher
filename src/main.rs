use std::error::Error;
use tokio::runtime::{Builder, Runtime};
use crate::mc::Minecraft;

mod mc;
mod downloader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Builder::new_current_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;

    rt.block_on(async {
        let mc = Minecraft::new().await;

        let res = mc.download_not_exists().await;
        match res {
            Ok(_) => {
                println!("Launched Minecraft")
            }
            Err(_) => {
                println!("Cannot download file")
            }
        }

        mc.launch().await;
    });

    Ok(())
}