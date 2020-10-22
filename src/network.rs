use async_std::net::UdpSocket;
use log::{error, info, warn};

pub struct LockServer;

#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("数据长度不足")]
    TooSmallPacket,
    #[error("未指定的指令")]
    UnexpectedCommand,
}

// fn read_message(mut stream: TcpStream) {
//
// }
//
// fn execute_message(message: &Vec<u8>, size: usize) -> Result<(), String> {
//     if size < 5 {
//         return Err(ProtocolError::TooSmallPacket.into());
//     }
//
//     match &message[0] {
//         0x00 => {
//             let uid = message[1..=4];
//
//         },
//         _ => return Err(ProtocolError::UnexpectedCommand.into()),
//     }
//     Ok(())
// }

#[inline]
pub fn bytes_to_u32(bytes: &[u8; 4]) -> u32 {
    return ((bytes[0] as u32) << 24)
        + ((bytes[1] as u32) << 16)
        + ((bytes[2] as u32) << 8)
        + bytes[3] as u32;
}

impl LockServer {
    pub async fn start(lock_addr: &str) -> anyhow::Result<()> {
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
