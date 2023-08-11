use std::fmt;

use crate::frame::FrameHeader;


/// DATA Frame flags.
#[derive(Debug, PartialEq)]
pub enum DataFlag {
    EndStream,
    Padded,
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
    data: Vec<u8>,
    stream: u32,
    flags: Vec<DataFlag>,
}

impl Data {
    /// Deserialize a DATA frame from a frame header and a payload.
    /// 
    /// # Arguments
    /// 
    /// * `header` - The frame header.
    /// * `payload` - The frame payload.
    pub fn deserialize(header: FrameHeader, payload: Vec<u8>) -> Self {
        let mut flags = Vec::new();

        if header.flags() & 0x1 != 0 {
            flags.push(DataFlag::EndStream);
        }

        if header.flags() & 0x8 != 0 {
            flags.push(DataFlag::Padded);
        }

        if flags.contains(&DataFlag::Padded) {
            let pad_length = payload[0] as usize;
            let data = payload[1..payload.len() - pad_length + 1].to_vec();

            Self {
                data,
                stream: header.stream_identifier(),
                flags,
            }
        } else {
            Self {
                data: payload,
                stream: header.stream_identifier(),
                flags,
            }
        }
    }
}

impl fmt::Display for Data {
    /// Format a DATA frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Data Frame\n")?;
        write!(f, "Stream Identifier: {}\n", self.stream)?;
        write!(f, "Flags: {:?}\n", self.flags)?;
        write!(f, "Data: {}\n", String::from_utf8_lossy(&self.data))
    }
}
