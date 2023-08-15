use std::fmt;

use crate::{error::Http2Error, frame::FrameHeader};

/// WINDOW_UPDATE Frame payload.
///
/// The WINDOW_UPDATE frame (type=0x8) is used to implement flow control.
///
///  +-+-------------------------------------------------------------+
///  |R|              Window Size Increment (31)                     |
///  +-+-------------------------------------------------------------+
#[derive(Debug)]
pub struct WindowUpdate {
    reserved: bool,
    window_size_increment: u32,
}


impl WindowUpdate {
    /// Deserialize a WINDOW_UPDATE frame from a frame header and a payload.
    ///
    /// # Arguments
    ///
    /// * `header` - The frame header.
    /// * `payload` - The frame payload.
    pub fn deserialize(header: &FrameHeader, payload: Vec<u8>) -> Result<Self, Http2Error> {
        // Check if the payload has the correct length.
        if payload.len() != 4 {
            return Err(Http2Error::FrameError(format!(
                "WINDOW_UPDATE frame payload must be 4 bytes: received {} bytes",
                payload.len()
            )));
        }

        let reserved = (payload[0] >> 7) != 0;

        let window_size_increment: u32 =
            u32::from_be_bytes([payload[0] & 0b0111_1111, payload[1], payload[2], payload[3]]);

        Ok(Self {
            reserved,
            window_size_increment,
        })
    }
}

impl fmt::Display for WindowUpdate {
    /// Format a WINDOW_UPDATE frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WINDOW_UPDATE Frame\n")?;
        write!(f, "Reserved: {}\n", self.reserved)?;
        write!(f, "Window Size Increment: {}", self.window_size_increment)?;
        Ok(())
    }
}
