mod network;
mod user;

use crate::network::LockServer;
use log::error;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    if let Err(e) = LockServer::start("0.0.0.0:10292").await {
        error!("Lock server exited because {}", e);
    }
    Ok(())
}
