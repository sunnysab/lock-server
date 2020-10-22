use async_std::io::{BufReader, BufWriter};
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use futures_util::io::AsyncReadExt;
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
        let server_socket = TcpListener::bind(lock_addr).await?;
        info!("Listening on {}", server_socket.local_addr()?);

        while let (mut stream, peer) = server_socket.accept().await? {
            async_std::task::spawn(async move {
                let (rx, tx) = stream.split();

                // let buf: BufReader<ReadHalf<TcpStream>> = BufReader::new(rx);
                // let buf_w: BufWriter<WriteHalf<TcpStream>> = BufWriter::new(tx);
                let mut buffer = vec![0u8; 10];
                info!("Accepted connection from {}", peer.ip().to_string());

                loop {
                    match stream.read(&mut buffer).await {
                        Ok(0) => {
                            warn!("Connection lost.");
                            break;
                        }
                        Ok(size) => {
                            info!("Packet received: {:?}", &buffer[..size]);
                            if size == 5 {
                                stream.write_all(&[0x01, 0x00, 0x00, 0x00, 0x00]).await;
                            }
                        }
                        Err(e) => {
                            warn!("Failed to read from tcp stream: {}", e.to_string());
                            break;
                        }
                    }
                }
            });
        }
        Ok(())
    }
}
