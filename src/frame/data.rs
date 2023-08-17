use std::fmt;

use crate::error::Http2Error;
use crate::frame::{FrameFlag, FrameHeader};

#[derive(Debug, PartialEq)]
pub struct DataFrame {
    pub stream_id: u32,
    pub end_stream: bool,
    pub data: Vec<u8>,
}

impl DataFrame {
    pub fn deserialize_flags(byte: u8) -> Vec<FrameFlag> {
        let mut frame_flags = Vec::new();

        if (byte & 0x01) != 0 {
            frame_flags.push(FrameFlag::EndStream);
        }

        if (byte & 0x08) != 0 {
            frame_flags.push(FrameFlag::Padded);
        }

        frame_flags
    }

    pub fn deserialize(
        frame_header: &FrameHeader,
        bytes: &mut Vec<u8>,
    ) -> Result<Self, Http2Error> {
        // Deserialize the flags from the header.
        let frame_flags: Vec<FrameFlag> = DataFrame::deserialize_flags(frame_header.frame_flags());

        // Handle the padding if needed.
        if frame_flags.contains(&FrameFlag::Padded) {
            let pad_length = bytes[0] as usize;

            // Check that the padding length is not 0.
            if pad_length == 0 {
                return Err(Http2Error::FrameError(
                    "Padding length invalid: found 0".to_string(),
                ));
            }
            *bytes = bytes[1..frame_header.payload_length() as usize - pad_length].to_vec();
        }

        Ok(Self {
            stream_id: frame_header.stream_identifier(),
            end_stream: frame_flags.contains(&FrameFlag::EndStream),
            data: bytes.clone(),
        })
    }
}

impl fmt::Display for DataFrame {
    /// Format a DATA frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DATA\n")?;
        write!(f, "Stream Identifier: {}\n", self.stream_id)?;
        write!(f, "End Stream: {}\n", self.end_stream)?;
        write!(f, "Data: {}\n", String::from_utf8_lossy(&self.data))
    }
}
