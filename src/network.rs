use crate::auth::{CardIdType, UserManager};
use crate::protocol::{LockCommand, LockRequest};
use anyhow::Result;
use async_std::net::UdpSocket;
use log::{error, info, warn};

type EnvData = sqlx::SqlitePool;

pub struct LockServer;

async fn on_unlock_request(env: &EnvData, card_id: CardIdType) -> Result<Option<Vec<u8>>> {
    let manager = UserManager::new(env);
    if let Some(u) = manager.query_by_card(card_id).await? {
        return Ok(Some(LockCommand::Unlock(u.card as u32).into()));
    }
    Ok(None)
}

async fn execute_message(env: &EnvData, message_in: Vec<u8>) -> Result<Option<Vec<u8>>> {
    /* Process the lock request and respond to the lock. */
    return match LockRequest::from_message(message_in)? {
        LockRequest::Unlock(card_id) => on_unlock_request(env, card_id as i64).await,
    };
}

impl LockServer {
    pub async fn start(lock_addr: &str, env: EnvData) -> Result<()> {
        let server = UdpSocket::bind(lock_addr).await?;
        info!("Listening on {}", server.local_addr()?);

        let mut buffer = vec![0u8; 10];

        loop {
            match server.recv_from(&mut buffer).await {
                Ok((size, peer)) => {
                    info!(
                        "Packet received from [{}]: {:?}",
                        peer.to_string(),
                        &buffer[..size]
                    );

                    if let Ok(resp) = execute_message(&env, buffer[..size].to_vec()).await {
                        if let Some(content) = resp {
                            info!("Command: Open door");
                            server.send_to(&content, peer).await.unwrap_or_else(|e| {
                                warn!("Failed to send command to the lock: {}", e);
                                0
                            });
                            continue;
                        }
                    }
                    warn!("Refused to open the door");
                }
                Err(e) => {
                    error!("Failed to recv from socket: {}", e);
                    break;
                }
            }
        }
        warn!("Loop exit.");
        Ok(())
    }
}
