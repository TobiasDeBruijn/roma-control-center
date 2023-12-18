mod command;
mod packet;

use thiserror::Error;
pub use command::*;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    #[error("Packet has an invalid length. Expected (at least) {expected}, but got {got}")]
    InvalidLength {
        expected: usize,
        got: usize,
    },
}

impl ParseError {
    pub fn new_invalid_length(expected: usize, got: usize) -> ParseError {
        Self::InvalidLength {
            expected,
            got,
        }
    }
}

pub trait Packet
where
    Self: Sized,
{
    fn serialize(self) -> Vec<u8>;
    fn deserialize(v: &[u8]) -> Result<Self, ParseError>;
}
