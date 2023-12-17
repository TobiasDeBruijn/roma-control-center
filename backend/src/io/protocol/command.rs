use crate::io::protocol::packet::{decode_u16, decode_u32, encode_u16, encode_u32};
use crate::io::protocol::{Packet, ParseError};

pub struct CommandPacket {
    serial_number: u32,
    ack_id: u16,
    flags: Vec<CommandFlags>,
    command: Command,
}

pub enum CommandFlags {
    IsAck,
}

pub enum Command {
    TurnOnOff { on: bool, relay_index: u8 },
}

impl Packet for CommandPacket {
    fn serialize(self) -> Vec<u8> {
        let sn = encode_u32(self.serial_number);
        let ack = encode_u16(self.ack_id);
        let header = vec![
            0x01,
            sn[0],
            sn[1],
            sn[2],
            sn[3],
            ack[0],
            ack[1],
            self.encode_flags(),
        ];

        [header, self.encode_command_body()].concat()
    }

    fn deserialize(v: &[u8]) -> Result<Self, ParseError> {
        let serial_number = decode_u32(&v[0..=4]);
        let ack_id = decode_u16(&v[5..=6]);
        let flags = Self::decode_flags(v[7]);
        let cmd_type = decode_u16(&v[8..=9]);

        let command = match cmd_type {
            0x01 => Command::TurnOnOff {
                on: v[10] == 1,
                relay_index: v[11],
            },
            _ => return Err(ParseError::InvalidValue("Invalid command type".to_string())),
        };

        Ok(Self {
            serial_number,
            ack_id,
            flags,
            command,
        })
    }
}

impl CommandPacket {
    fn decode_flags(v: u8) -> Vec<CommandFlags> {
        // Build a Vec of Option<CommandFlag>, filter out the None's at the end.
        vec![((v & 0x80) != 0).then(|| CommandFlags::IsAck)]
            .into_iter()
            .filter_map(|v| v)
            .collect()
    }

    fn encode_command_body(&self) -> Vec<u8> {
        let cmd_type = encode_u16(self.encode_command_type());
        match self.command {
            Command::TurnOnOff { on, relay_index } => {
                vec![
                    cmd_type[0],
                    cmd_type[1],
                    if on { 1 } else { 0 },
                    relay_index,
                ]
            }
        }
    }

    fn encode_command_type(&self) -> u16 {
        match self.command {
            Command::TurnOnOff { .. } => 0x01,
        }
    }

    fn encode_flags(&self) -> u8 {
        self.flags.iter().fold(0_u8, |acc, flag| match flag {
            CommandFlags::IsAck => acc | 0x80,
        })
    }
}
