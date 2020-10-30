use crate::util::bytes_to_u32;
use std::convert::TryFrom;

const TYPE_UNLOCK_REQUEST: u8 = 0;
const TYPE_UNLOOK_COMMAND: u8 = 1;
const TYPE_BUTTON_PUSHED: u8 = 3;

/* Protocol errors */
#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    // #[error("数据长度不足")]
    // TooSmallPacket,
    #[error("未指定的指令")]
    UnexpectedCommand,
}

pub enum LockRequest {
    /// Request to unlock by card
    Unlock(u32),
    /// Unlock button report
    ButtonReport,
}

pub enum LockCommand {
    /// Unlock
    Unlock(u32),
}

impl LockRequest {
    /// Resolve requests from the lock
    pub fn from_message(message_in: Vec<u8>) -> std::result::Result<Self, ProtocolError> {
        let packet: Self = match message_in[0] {
            TYPE_UNLOCK_REQUEST => {
                LockRequest::Unlock(bytes_to_u32(<&[u8; 4]>::try_from(&message_in[1..=4]).unwrap()))
            }
            TYPE_BUTTON_PUSHED => LockRequest::ButtonReport,
            _ => return Err(ProtocolError::UnexpectedCommand.into()),
        };
        Ok(packet)
    }
}

impl LockCommand {
    /// Pack an unlock command packet
    fn unlock(card_id: u32) -> Self {
        Self::Unlock(card_id)
    }
}

impl Into<Vec<u8>> for LockCommand {
    fn into(self) -> Vec<u8> {
        match self {
            LockCommand::Unlock(card_id) => vec![
                0x01u8,
                (card_id >> 24) as u8,
                (card_id >> 16 & 0xFF) as u8,
                (card_id >> 8 & 0xFF) as u8,
                (card_id & 0xFF) as u8,
            ],
        }
    }
}
