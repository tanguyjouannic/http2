use std::fmt;

use crate::error::Http2Error;
use crate::frame::{FrameFlag, FrameHeader};

#[derive(Debug, PartialEq)]
pub struct PingFrame {
    ack: bool,
    opaque_data: Vec<u8>,
}

impl PingFrame {
    pub fn deserialize_flags(byte: u8) -> Vec<FrameFlag> {
        let mut frame_flags = Vec::new();

        if (byte & 0x01) != 0 {
            frame_flags.push(FrameFlag::Ack);
        }

        frame_flags
    }

    pub fn deserialize(
        frame_header: &FrameHeader,
        bytes: &mut Vec<u8>,
    ) -> Result<Self, Http2Error> {
        // Check if the bytes vector contains at least 8 bytes.
        if bytes.len() < 8 {
            return Err(Http2Error::NotEnoughBytes(format!(
                "PING frame needs at least 8 bytes, found {}",
                bytes.len()
            )));
        }

        // Deserialize the flags from the header.
        let flags: Vec<FrameFlag> = PingFrame::deserialize_flags(frame_header.frame_flags());

        Ok(PingFrame {
            ack: flags.contains(&FrameFlag::Ack),
            opaque_data: bytes[0..8].to_vec(),
        })
    }
}

impl fmt::Display for PingFrame {
    /// Format a PING frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PING\n")?;
        write!(f, "Ack: {}\n", self.ack)?;
        write!(f, "Opaque Data: {:?}\n", self.opaque_data)
    }
}
