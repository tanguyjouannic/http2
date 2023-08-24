use std::fmt;

use crate::error::Http2Error;
use crate::frame::FrameHeader;

/// WINDOW_UPDATE Frame.
///
/// The WINDOW_UPDATE frame (type=0x8) is used to implement flow control.
///
/// +-+-------------------------------------------------------------+
/// |R|              Window Size Increment (31)                     |
/// +-+-------------------------------------------------------------+
#[derive(Debug, PartialEq)]
pub struct WindowUpdateFrame {
    stream_id: u32,
    reserved: bool,
    window_size_increment: u32,
}

impl WindowUpdateFrame {
    /// Deserialize a WINDOW_UPDATE frame.
    /// 
    /// The operation is destructive for the bytes vector.
    /// 
    /// # Arguments
    /// 
    /// * `frame_header` - A reference to a FrameHeader.
    /// * `bytes` - A mutable reference to a bytes vector.
    pub fn deserialize(
        frame_header: &FrameHeader,
        bytes: &mut Vec<u8>,
    ) -> Result<Self, Http2Error> {
        // Check if the bytes has the right length.
        if bytes.len() != frame_header.payload_length() as usize {
            return Err(Http2Error::FrameError(format!(
                "Expected {} bytes for WINDOW_UPDATE frame, found {}",
                frame_header.payload_length(),
                bytes.len()
            )));
        }

        Ok(WindowUpdateFrame {
            stream_id: frame_header.stream_id(),
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
