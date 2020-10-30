use crate::auth::{CardIdType, UserManager};
use crate::protocol::{LockCommand, LockRequest};
use anyhow::Result;
use async_std::net::UdpSocket;
use log::{error, info, warn};

type EnvData = sqlx::SqlitePool;

/*
 * Processing function for the request from the lock.
 *
 * on_unlock_request
 * on_button_pushed
 */

/// On unlock request, the function tries to inquire from database
async fn on_unlock_request(env: &EnvData, card_id: CardIdType) -> Result<Option<Vec<u8>>> {
    let pool = env;
    let manager = UserManager::new(pool);

    if let Some(u) = manager.query_by_card(card_id).await? {
        // Return a response command package.
        return Ok(Some(LockCommand::Unlock(u.card as u32).into()));
    }
    // Return a None, and not to send back any message
    Ok(None)
}

/// On inner unlock report, the function write the event to the log
async fn on_button_pushed(env: &EnvData) -> Result<()> {
    // Write log
}

/// Dispatch lock requests, and call the corresponding processing function
async fn execute_message(env: &EnvData, message_in: Vec<u8>) -> Result<Option<Vec<u8>>> {
    /* Process the lock request and respond to the lock. */
    match LockRequest::from_message(message_in)? {
        LockRequest::Unlock(card_id) => on_unlock_request(env, card_id as i64).await,
        LockRequest::ButtonReport => on_button_pushed(env).await,
    }
}

/// Listen to a UDP address and run main event loop
pub async fn run(lock_addr: &str, env: EnvData) -> Result<()> {
    // Bind a UDP port
    let server = UdpSocket::bind(lock_addr).await?;
    info!("Listening on {}", server.local_addr()?);

    // Receive buffer
    let mut buffer = vec![0u8; 10];

    loop {
        let recv_result = server.recv_from(&mut buffer).await;
        if let Ok((size, peer)) = recv_result {
            info!("Packet received from [{:?}]: {:?}", peer, &buffer[..size]);

            let exec_result = execute_message(&env, buffer[..size].to_vec()).await;
            if let Ok(r) = exec_result {
                if let Some(content) = r {
                    if let Err(e) = server.send_to(&content, peer).await {
                        warn!("Failed to send command: {}", e);
                    } else {
                        info!("Command has sent");
                    } // Send command to lock
                } else {
                    info!("Response no information.");
                }
            } else if let Err(e) = exec_result {
                println!("Failed to parse or execute the request from lock: {}", e)
            }
            warn!("Refused to open the door");
        } else if let Err(e) = recv_result {
            error!("Failed to recv from socket: {}", e);
            break;
        }
    }
    warn!("Loop exit.");
    Ok(())
}
