use std::fmt;

use crate::error::Http2Error;
use crate::frame::FrameHeader;

#[derive(Debug, PartialEq)]
pub struct RstStreamFrame {
    pub stream_id: u32,
    pub error_code: u32,
}

impl RstStreamFrame {
    pub fn deserialize(
        frame_header: &FrameHeader,
        bytes: &mut Vec<u8>,
    ) -> Result<Self, Http2Error> {
        // Check if the bytes stream has at least 4 bytes.
        if bytes.len() < 4 {
            return Err(Http2Error::NotEnoughBytes(format!(
                "RST_STREAM frame needs at least 4 bytes, found {}",
                bytes.len()
            )));
        }

        // Retrieve the error code.
        let error_code = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

        // Remove the error code from the bytes stream.
        *bytes = bytes[4..].to_vec();

        Ok(Self {
            stream_id: frame_header.stream_identifier(),
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
