use std::fmt;

use crate::error::Http2Error;
use crate::frame::FrameHeader;

/// RST_STREAM Frame.
///
/// The RST_STREAM frame (type=0x3) allows for immediate termination of a
/// stream.  RST_STREAM is sent to request cancellation of a stream or to
/// indicate that an error condition has occurred.
///
/// +---------------------------------------------------------------+
/// |                        Error Code (32)                        |
/// +---------------------------------------------------------------+
#[derive(Debug, PartialEq)]
pub struct RstStreamFrame {
    pub stream_id: u32,
    pub error_code: u32,
}

impl RstStreamFrame {
    /// Deserialize a RST_STREAM frame.
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
                "Expected {} bytes for RST_STREAM frame, found {}",
                frame_header.payload_length(),
                bytes.len()
            )));
        }
        // Retrieve the error code.
        let error_code = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

        // Remove the error code from the bytes stream.
        *bytes = bytes[4..].to_vec();

        Ok(Self {
            stream_id: frame_header.stream_id(),
            error_code,
        })
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
