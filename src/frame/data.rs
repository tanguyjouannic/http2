use std::fmt;

use crate::{frame::FrameHeader, error::Http2Error};


/// DATA Frame flags.
#[derive(Debug, PartialEq)]
pub enum DataFlag {
    EndStream,
    Padded,
}

impl DataFlag {
    /// Parse the flags from a byte.
    /// 
    /// # Arguments
    /// 
    /// * `byte` - The byte to parse the flags from.
    pub fn parse_flags(byte: u8) -> Vec<DataFlag> {
        let mut flags: Vec<DataFlag> = Vec::new();

        if byte & 0x1 != 0 {
            flags.push(DataFlag::EndStream);
        }

        if byte & 0x8 != 0 {
            flags.push(DataFlag::Padded);
        }

        flags
    }
}

/// DATA Frame structure.
///
/// DATA frames (type=0x0) convey arbitrary, variable-length sequences of
/// octets associated with a stream. One or more DATA frames are used,
/// for instance, to carry HTTP request or response payloads.
///
/// DATA frames MAY also contain padding. Padding can be added to DATA
/// frames to obscure the size of messages. Padding is a security
/// feature
///
///  +---------------+
///  |Pad Length? (8)|
///  +---------------+-----------------------------------------------+
///  |                            Data (*)                         ...
///  +---------------------------------------------------------------+
///  |                           Padding (*)                       ...
///  +---------------------------------------------------------------+
#[derive(Debug)]
pub struct Data {
    header: FrameHeader,
    data: Vec<u8>,
    parsed_flags: Vec<DataFlag>,
}

impl Data {
    /// Deserialize a DATA frame from a frame header and a payload.
    /// 
    /// # Arguments
    /// 
    /// * `header` - The frame header.
    /// * `payload` - The frame payload.
    pub fn deserialize(header: FrameHeader, mut payload: Vec<u8>) -> Result<Self, Http2Error> {
        // Parse the flags from the header.
        let parsed_flags: Vec<DataFlag> = DataFlag::parse_flags(header.flags());

        if parsed_flags.contains(&DataFlag::Padded) {
            let pad_length = payload[0] as usize;

            // Check that the padding length is not 0.
            if pad_length == 0 {
                return Err(Http2Error::FrameError("Padding length is 0".to_string()));
            }
            payload = payload[1..payload.len() - pad_length].to_vec();
        }

        Ok(Self {
            header,
            data: payload,
            parsed_flags,
        })
    }
}

impl fmt::Display for Data {
    /// Format a DATA frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Data Frame\n")?;
        write!(f, "Parsed Flags: {:?}\n", self.parsed_flags)?;
        write!(f, "Data: {}\n", String::from_utf8_lossy(&self.data))
    }
}
