use std::fmt;

use crate::error::Http2Error;
use crate::frame::FrameHeader;

#[derive(Debug, PartialEq)]
pub struct GoAwayFrame {
    reserved: bool,
    last_stream_id: u32,
    error_code: u32,
    debug_data: Option<Vec<u8>>,
}

impl GoAwayFrame {
    pub fn deserialize(
        frame_header: &FrameHeader,
        bytes: &mut Vec<u8>,
    ) -> Result<Self, Http2Error> {
        // Check if the bytes vector contains at least 8 bytes.
        if bytes.len() < 8 {
            return Err(Http2Error::NotEnoughBytes(format!(
                "GOAWAY frame needs at least 8 bytes, found {}",
                bytes.len()
            )));
        }

        // Retrieve the frame fields.
        let reserved: bool = (bytes[0] >> 7) != 0;
        let last_stream_id: u32 =
            u32::from_be_bytes([bytes[0] & 0x7F, bytes[1], bytes[2], bytes[3]]);
        let error_code: u32 = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let debug_data: Option<Vec<u8>> = if frame_header.payload_length() > 8 {
            Some(bytes[8..frame_header.payload_length() as usize].to_vec())
        } else {
            None
        };

        Ok(GoAwayFrame {
            reserved,
            last_stream_id,
            error_code,
            debug_data,
        })
    }
}

impl fmt::Display for GoAwayFrame {
    /// Format a GOAWAY frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GOAWAY\n")?;
        write!(f, "Reserved: {}\n", self.reserved)?;
        write!(f, "Last Stream ID: {}\n", self.last_stream_id)?;
        write!(f, "Error Code: {}\n", self.error_code)?;
        match self.debug_data {
            Some(ref debug_data) => {
                write!(f, "Debug Data: {}\n", String::from_utf8_lossy(debug_data))
            }
            None => write!(f, "Debug Data: None\n"),
        }
    }
}
