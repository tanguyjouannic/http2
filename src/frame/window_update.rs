use std::fmt;

use crate::error::Http2Error;
use crate::frame::FrameHeader;

#[derive(Debug, PartialEq)]
pub struct WindowUpdateFrame {
    stream_id: u32,
    reserved: bool,
    window_size_increment: u32,
}

impl WindowUpdateFrame {
    pub fn deserialize(
        frame_header: &FrameHeader,
        bytes: &mut Vec<u8>,
    ) -> Result<Self, Http2Error> {
        // Check if the bytes vector contains at least 4 bytes.
        if bytes.len() < 4 {
            return Err(Http2Error::NotEnoughBytes(format!(
                "WINDOW_UPDATE frame needs at least 4 bytes, found {}",
                bytes.len()
            )));
        }

        Ok(WindowUpdateFrame {
            stream_id: frame_header.stream_identifier(),
            reserved: (bytes[0] >> 7) != 0,
            window_size_increment: u32::from_be_bytes([
                bytes[0] & 0x7F,
                bytes[1],
                bytes[2],
                bytes[3],
            ]),
        })
    }
}

impl fmt::Display for WindowUpdateFrame {
    /// Format a WINDOW_UPDATE frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WINDOW_UPDATE\n")?;
        write!(f, "Stream Identifier: {}\n", self.stream_id)?;
        write!(f, "Reserved: {}\n", self.reserved)?;
        write!(f, "Window Size Increment: {}\n", self.window_size_increment)
    }
}
