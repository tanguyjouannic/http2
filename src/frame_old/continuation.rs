use std::fmt;

use crate::{error::Http2Error, frame::FrameHeader, header::{list::HeaderList, table::HeaderTable}};

/// CONTINUATION Frame flags.
#[derive(Debug, PartialEq)]
pub enum ContinuationFlag {
    EndHeaders,
}

impl ContinuationFlag {
    /// Deserialize the flags from a byte.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to deserialize the flags from.
    pub fn deserialize(byte: u8) -> Vec<ContinuationFlag> {
        let mut flags: Vec<ContinuationFlag> = Vec::new();

        if byte & 0x4 != 0 {
            flags.push(ContinuationFlag::EndHeaders);
        }

        flags
    }
}

/// CONTINUATION Frame payload.
///
/// The CONTINUATION frame (type=0x9) is used to continue a sequence of
/// header block fragments. Any number of CONTINUATION frames can be 
/// sent, as long as the preceding frame is on the same stream and is a 
/// HEADERS, PUSH_PROMISE, or CONTINUATION frame without the 
/// END_HEADERS flag set.
///
///  +---------------------------------------------------------------+
///  |                   Header Block Fragment (*)                 ...
///  +---------------------------------------------------------------+
#[derive(Debug)]
pub struct Continuation {
    flags: Vec<ContinuationFlag>,
    header_list: HeaderList,
}

impl Continuation {
    /// Deserialize a CONTINUATION frame from a frame header and a payload.
    ///
    /// # Arguments
    ///
    /// * `header` - The frame header.
    /// * `payload` - The frame payload.
    pub fn deserialize(
        header: &FrameHeader,
        mut payload: Vec<u8>,
        header_table: &mut HeaderTable,
    ) -> Result<Self, Http2Error> {
        // Check if the payload has the correct length.
        if header.payload_length() as usize != payload.len() {
            return Err(Http2Error::FrameError(format!(
                "CONTINUATION frame payload length is incorrect: expected {}, received {}",
                header.payload_length(),
                payload.len()
            )));
        }

        // Deserialize the flags from the header.
        let flags: Vec<ContinuationFlag> = ContinuationFlag::deserialize(header.flags());

        // Try to decode the header list.
        let header_list = HeaderList::decode(&mut payload, header_table)?;

        Ok(Self {
            flags,
            header_list,
        })
    }
}

impl fmt::Display for Continuation {
    /// Format a CONTINUATION frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {  
        write!(f, "CONTINUATION\n")?;
        write!(f, "Flags: {:?}\n", self.flags)?;
        write!(f, "Header List:\n{}", self.header_list)?;
        Ok(())
    }
}