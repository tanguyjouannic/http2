use std::fmt;

use crate::{error::Http2Error, frame::FrameHeader};

/// RST_STREAM Frame payload.
///
/// The RST_STREAM frame (type=0x3) allows for immediate termination of a
/// stream.  RST_STREAM is sent to request cancellation of a stream or to
/// indicate that an error condition has occurred.
///
///  +---------------------------------------------------------------+
///  |                        Error Code (32)                        |
///  +---------------------------------------------------------------+
#[derive(Debug)]
pub struct RstStream {
    error_code: u32,
}

impl RstStream {
    /// Deserialize a RST_STREAM frame from a frame header and a payload.
    ///
    /// # Arguments
    ///
    /// * `header` - The frame header.
    /// * `payload` - The frame payload.
    pub fn deserialize(header: &FrameHeader, payload: Vec<u8>) -> Result<Self, Http2Error> {
        // Check if the payload has the correct length.
        if payload.len() != 4 {
            return Err(Http2Error::FrameError(format!(
                "Invalid payload length for PRIORITY frame: {}",
                payload.len()
            )));
        }

        Ok(Self {
            error_code: u32::from_be_bytes([payload[0], payload[1], payload[2], payload[3]]),
        })
    }
}

impl fmt::Display for RstStream {
    /// Format a DATA frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RST_STREAM Frame\n")?;
        write!(f, "Error Code: {}", self.error_code)?;
        Ok(())
    }
}
