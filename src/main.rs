mod auth;
mod network;
mod protocol;
mod service;
mod util;

use crate::network::LockServer;
use log::error;
use sqlx::SqlitePool;
use std::time::Duration;

type EnvData = sqlx::SqlitePool;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    /* Initialize a logger */
    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    /* Connect (Open) database */
    let pool = SqlitePool::new("lock.db").await.unwrap();

    async_std::task::spawn(async {
        if let Err(e) = service::serve().await {
            error!("Failed to run HTTP server: {}", e);
        }
    });

    async_std::task::spawn(async {
        if let Err(e) = LockServer::start("0.0.0.0:10292", pool).await {
            error!("Lock server exited because {}", e);
        }
    });

    loop {
        // Do nothing
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}
