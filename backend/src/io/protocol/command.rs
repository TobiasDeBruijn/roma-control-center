use crate::io::protocol::packet::{decode_u16, decode_u32, encode_u16, encode_u32};
use crate::io::protocol::{Packet, ParseError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommandPacket {
    serial_number: u32,
    ack_id: u16,
    flags: Vec<CommandFlags>,
    command: Command,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandFlags {
    IsAck,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
        // Check header length
        if v.len() < 10 {
            return Err(ParseError::new_invalid_length(10, v.len()))
        }

        let serial_number = decode_u32(&v[1..=4]);
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

#[cfg(test)]
mod test {
    use super::*;

    fn default_for_command(command: Command) -> CommandPacket {
        CommandPacket {
            serial_number: 0,
            ack_id: 0,
            flags: vec![],
            command,
        }
    }

    fn default_with_command_bytes(mut cmd_bytes: Vec<u8>) -> Vec<u8> {
        let mut acc = vec![
            0x1,            // Packet type
            0, 0, 0, 0,     // Serial
            0, 0,           // Ack ID
            0,              // Flags
        ];

        acc.append(&mut cmd_bytes);
        acc
    }

    #[test]
    fn serialize_header() {
        let v = CommandPacket {
            serial_number: 0x3B9ACA0E,
            ack_id: 0x2354,
            flags: vec![
                CommandFlags::IsAck,
            ],
            command: Command::TurnOnOff { // Not relevant for this test
                on: true,
                relay_index: 0,
            }
        }.serialize();

        // Type
        assert_eq!(v[0], 0x01);

        // Serial number
        assert_eq!(v[1], 0x3B);
        assert_eq!(v[2], 0x9A);
        assert_eq!(v[3], 0xCA);
        assert_eq!(v[4], 0x0E);

        // ACK id
        assert_eq!(v[5], 0x23);
        assert_eq!(v[6], 0x54);

        // Flags
        assert_eq!(v[7], 0b1000_0000);
    }

    #[test]
    fn deserialize_header() {
        let v = CommandPacket {
            serial_number: 0x3B9ACAFE,
            ack_id: 0x2354,
            flags: vec![
                CommandFlags::IsAck,
            ],
            command: Command::TurnOnOff {
                on: true,
                relay_index: 0,
            }
        };

        let bytes = vec![
            0x01, // Packet type
            0x3B, // Serial number
            0x9A,
            0xCA,
            0xFE,
            0x23, // ACK id
            0x54,
            0b1000_0000, // Flags
            0x00, // Command type
            0x01,
            0x01, // On: true,
            0,    // Relay index
        ];

        let de = CommandPacket::deserialize(&bytes).expect("Failed to parse");

        assert_eq!(de, v);
    }

    #[test]
    fn serialize_on_off() {
        let v = default_for_command(Command::TurnOnOff {
            relay_index: 123,
            on: true,
        }).serialize();

        // Type
        assert_eq!(v[8], 0);
        assert_eq!(v[9], 0x1);

        // Data
        assert_eq!(v[10], 1);
        assert_eq!(v[11], 123);
    }

    #[test]
    fn deserialize_on_off() {
        let bytes = default_with_command_bytes(vec![
            0x00, // Command type
            0x01,
            1,    // on: true
            123,  // Relay index
        ]);

        let de = CommandPacket::deserialize(&bytes).expect("Failed to parse");

        match de.command {
            Command::TurnOnOff { on, relay_index } => {
                assert!(on);
                assert_eq!(relay_index, 123);
            },
            #[allow(unreachable_patterns)] // As of writing this test, no other command types exist yet.
            _ => panic!("Invalid command type"),
        };
    }
}