use crate::auth::{CardIdType, User, UserManager};
use crate::protocol::{LockCommand, LockRequest};
use crate::util::bytes_to_u32;
use crate::EnvData;
use anyhow::Result;
use async_std::net::UdpSocket;
use log::{error, info, warn};

pub struct LockServer;

async fn on_unlock_request(env: EnvData, card_id: CardIdType) -> Result<Option<Vec<u8>>> {
    let manager = UserManager::new(env);
    if let Some(u) = manager.query_by_card(card_id).await? {
        return Ok(Some(LockCommand::Unlock(u.card as u32).into()));
    }
    Ok(None)
}

async fn execute_message(env: EnvData, message_in: Vec<u8>) -> Result<Option<Vec<u8>>> {
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
                    if size == 5 {
                        let response_result =
                            server.send_to(&[0x01, 0x00, 0x00, 0x00, 0x00], peer).await;
                        if let Err(e) = response_result {
                            warn!("Failed responding to {}: {}", peer.to_string(), e);
                        }
                    }
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
