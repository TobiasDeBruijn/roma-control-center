use std::path::Path;
use thiserror::Error;
use crate::io::protocol;
use crate::io::protocol::{CommandPacket, Packet};
use crate::io::rfm_communicator::RfmCommunicator;

pub type TransceiverResult<T> = Result<T, TransceiverError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Command(CommandPacket),
}

impl Message {
    pub fn into_inner(self) -> impl Packet {
        match self {
            Self::Command(c) => c,
        }
    }
}

#[derive(Debug, Error)]
pub enum TransceiverError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Parse(#[from] protocol::ParseError),
    #[error("Unknown packet type '0x{0:X}'")]
    UnknownPacketType(u8),
}

pub struct MessageTransceiver {
    io: RfmCommunicator,
}

impl MessageTransceiver {
    pub fn new() -> TransceiverResult<Self> {
        Ok(Self {
            io: RfmCommunicator::new()?,
        })
    }

    pub fn new_with_port<P: AsRef<Path>>(serial_port: P) -> TransceiverResult<Self> {
        Ok(Self {
            io: RfmCommunicator::new_with_name(serial_port)?,
        })
    }

    pub fn try_receive(&mut self) -> TransceiverResult<Option<Message>> {
        let bytes = self.io.try_read()?;
        if bytes.is_empty() {
            return Ok(None);
        }

        Ok(Some(match bytes[0] {
            0x01 => Message::Command(CommandPacket::deserialize(&bytes)?),
            ty @ _=> return Err(TransceiverError::UnknownPacketType(ty))
        }))
    }

    pub fn try_send<M: Packet>(&mut self, message: M) -> TransceiverResult<()> {
        let bytes = message.serialize();
        self.io.try_write(&bytes)?;
        Ok(())
    }
}