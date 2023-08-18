use std::fmt;

use crate::error::Http2Error;
use crate::frame::{FrameHeader, FrameFlag};

#[derive(Debug, PartialEq)]
pub struct PingFrame {
    pub opaque_data: Vec<u8>,
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
            return Err(Http2Error::NotEnoughBytes(
                format!("PING frame needs at least 8 bytes, found {}", bytes.len()),
            ));
        }

        // Deserialize the flags from the header.
        let flags: Vec<FrameFlag> = SettingsFrame::deserialize_flags(frame_header.frame_flags());

        // Retrieve the opaque data.
        
    }
}

impl fmt::Display for RstStreamFrame {
    /// Format a RST_STREAM frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RST_STREAM\n")?;
        write!(f, "Stream Identifier: {}\n", self.stream_id)?;
        write!(f, "Error Code: {}\n", self.error_code)
    }
}
