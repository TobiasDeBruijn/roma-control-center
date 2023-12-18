use std::io;
use std::path::Path;
use serial2::SerialPort;

pub struct RfmCommunicator {
    port: SerialPort,
}

impl RfmCommunicator {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            port: Self::serial_port("/dev/ttyS0")?,
        })
    }

    pub fn new_with_name<P: AsRef<Path>>(name: P) -> io::Result<Self> {
        Ok(Self {
            port: Self::serial_port(name)?
        })
    }

    fn serial_port<P: AsRef<Path>>(name: P) -> io::Result<SerialPort> {
        SerialPort::open(name, 9600)
    }

    pub fn try_read(&mut self) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        let mut in_message = false;

        let bytes = loop {
            let mut read_buf = [0_u8; 1];
            let num_bytes_read = self.port.read(&mut read_buf)?;

            if num_bytes_read == 0 {
                // No more bytes are available
                break buf;
            }

            let byte = read_buf[0];

            /*
            Message format (hex): AE EA (payload) CA AC
            We first try to find the start bytes, then we continue reading
            until we find the stop bytes. We then remove the start and stop bytes from the buffer
            and thus return only the payload.
             */

            // Check for start bytes
            if !in_message {
                if let Some(previous_byte) = buf.last() {
                    if *previous_byte == 0xAE && byte == 0xEA {
                        in_message = true;
                        continue;
                    }
                }
            }

            // Check for stop bytes
            if in_message {
                if let Some(previous_byte) = buf.last() {
                    if *previous_byte == 0xCA && byte == 0xAC {
                        // Remove start bytes and stop byte
                        let mut buf = buf.into_iter()
                            .skip(2) // Skip start bytes
                            .collect::<Vec<_>>();
                        buf.pop(); // Remove first of stop bytes

                        break buf;
                    }
                }
            }

            // Safety, shouldnt happen in practice.
            if buf.len() > 64 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "No message could be found in bytes received"));
            }

            buf.push(byte);
        };

        Ok(bytes)
    }

    pub fn try_write(&mut self, bytes: &[u8]) -> io::Result<()> {
        // Add the start and stop bytes around the data
        let buf: Vec<u8> = [&[0xAE_u8, 0xEA], bytes, &[0xCA_u8, 0xAC]].concat();

        self.port.write_all(&buf)?;
        Ok(())
    }
}