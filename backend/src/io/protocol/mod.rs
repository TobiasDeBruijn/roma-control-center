mod command;
mod packet;

pub enum ParseError {
    InvalidValue(String),
}

pub trait Packet
where
    Self: Sized,
{
    fn serialize(self) -> Vec<u8>;
    fn deserialize(v: &[u8]) -> Result<Self, ParseError>;
}
