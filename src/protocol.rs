use crate::util::bytes_to_u32;

const TYPE_UNLOCK_REQUEST: u8 = 0;
const TYPE_UNLOCK_COMMAND: u8 = 1;
const TYPE_BUTTON_PUSHED: u8 = 3;

/* Protocol errors */
#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("Un expected request received.")]
    UnexpectedRequest,
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
            TYPE_UNLOCK_REQUEST => LockRequest::Unlock(bytes_to_u32(&message_in[1..=4])),
            TYPE_BUTTON_PUSHED => LockRequest::ButtonReport,
            _ => return Err(ProtocolError::UnexpectedRequest.into()),
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
            LockCommand::Unlock(_) => vec![0x01u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8],
        }
    }
}
