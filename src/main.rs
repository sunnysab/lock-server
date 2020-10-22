mod lock;
mod network;
mod service;
mod user;

use crate::network::LockServer;
use log::error;
use std::time::Duration;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    async_std::task::spawn(async {
        if let Err(e) = service::serve().await {
            error!("Failed to run HTTP server: {}", e);
        }
    });

    async_std::task::spawn(async {
        if let Err(e) = LockServer::start("0.0.0.0:10292").await {
            error!("Lock server exited because {}", e);
        }
    });

    loop {
        async_std::task::sleep(Duration::from_secs(1));
        // Do nothing
    }
}
