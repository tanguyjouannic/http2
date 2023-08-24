use std::fmt;

use crate::error::Http2Error;
use crate::frame::{FrameFlag, FrameHeader};

/// DATA Frame.
///
/// DATA frames (type=0x0) convey arbitrary, variable-length sequences of
/// octets associated with a stream. One or more DATA frames are used,
/// for instance, to carry HTTP request or response payloads.
///
/// DATA frames MAY also contain padding. Padding can be added to DATA
/// frames to obscure the size of messages. Padding is a security
/// feature
///
/// +---------------+
/// |Pad Length? (8)|
/// +---------------+-----------------------------------------------+
/// |                            Data (*)                         ...
/// +---------------------------------------------------------------+
/// |                           Padding (*)                       ...
/// +---------------------------------------------------------------+
#[derive(Debug, PartialEq)]
pub struct DataFrame {
    pub stream_id: u32,
    pub end_stream: bool,
    pub data: Vec<u8>,
}

impl DataFrame {
    /// Create a new DATA frame.
    /// 
    /// # Arguments
    /// 
    /// * `stream_id` - The stream identifier.
    /// * `end_stream` - A boolean indicating if the DATA frame is the last frame of the stream.
    /// * `data` - The data to send.
    pub fn new(stream_id: u32, end_stream: bool, data: Vec<u8>) -> Self {
        Self {
            stream_id,
            end_stream,
            data,
        }
    }

    /// Serialize a DATA frame.
    /// 
    /// Panic if the optional padding length is greater than 255.
    /// 
    /// # Arguments
    /// 
    /// * `padding` - An optional bytes padding with max length of 255.
    pub fn serialize(&self, padding: Option<Vec<u8>>) -> Vec<u8> {
        // Build the payload.
        let mut payload: Vec<u8> = Vec::new();
        match padding.clone() {
            Some(padding) => {
                // Panic if the padding length is greater than 255.
                if padding.len() > 255 {
                    panic!("Padding length greater than 255");
                }

                payload.push(padding.len() as u8);
                payload.append(&mut self.data.clone());
                payload.append(&mut padding.clone());
            },
            None => {
                payload.append(&mut self.data.clone());
            }
        }

        // Build the flags bit.
        let mut frame_flags: u8 = 0x0;
        if self.end_stream {
            frame_flags |= 0x01;
        }
        if padding.clone().is_some() {
            frame_flags |= 0x08;
        }

        // Build the header.
        let header = FrameHeader::new(
            payload.len() as u32,
            0x0, 
            frame_flags,
            false,
            self.stream_id, 
        );

        // Serialize the frame.
        let mut bytes: Vec<u8> = Vec::new();
        bytes.append(&mut header.serialize());
        bytes.append(&mut payload);

        bytes
    }

    /// Deserialize the flags from a byte.
    /// 
    /// # Arguments
    /// 
    /// * `byte` - The byte containing the flags.
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

    /// Deserialize a DATA frame.
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
                "Expected {} bytes for DATA frame, found {}",
                frame_header.payload_length(),
                bytes.len()
            )));
        }

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
            stream_id: frame_header.stream_id(),
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
