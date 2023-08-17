use std::fmt;

use crate::{
    error::Http2Error,
    frame::FrameHeader,
    header::{list::HeaderList, table::HeaderTable},
};

/// HEADERS Frame flags.
#[derive(Debug, PartialEq)]
pub enum HeadersFlag {
    EndStream,
    EndHeaders,
    Padded,
    Priority,
}

impl HeadersFlag {
    /// Deserialize the flags from a byte.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to deserialize the flags from.
    pub fn deserialize(byte: u8) -> Vec<HeadersFlag> {
        let mut flags: Vec<HeadersFlag> = Vec::new();

        if byte & 0x1 != 0 {
            flags.push(HeadersFlag::EndStream);
        }

        if byte & 0x4 != 0 {
            flags.push(HeadersFlag::EndHeaders);
        }

        if byte & 0x8 != 0 {
            flags.push(HeadersFlag::Padded);
        }

        if byte & 0x20 != 0 {
            flags.push(HeadersFlag::Priority);
        }

        flags
    }
}

/// HEADERS Frame payload.
///
/// The HEADERS frame (type=0x1) is used to open a stream (Section 5.1),
/// and additionally carries a header block fragment. HEADERS frames can
/// be sent on a stream in the "idle", "reserved (local)", "open", or
/// "half-closed (remote)" state.
///
///  +---------------+
///  |Pad Length? (8)|
///  +-+-------------+-----------------------------------------------+
///  |E|                 Stream Dependency? (31)                     |
///  +-+-------------+-----------------------------------------------+
///  |  Weight? (8)  |
///  +-+-------------+-----------------------------------------------+
///  |                   Header Block Fragment (*)                 ...
///  +---------------------------------------------------------------+
///  |                           Padding (*)                       ...
///  +---------------------------------------------------------------+
#[derive(Debug)]
pub struct Headers {
    header_list: HeaderList,
    flags: Vec<HeadersFlag>,
    exclusivity: Option<bool>,
    stream_dependency: Option<u32>,
    weight: Option<u8>,
}

impl Headers {
    /// Deserialize a HEADERS frame from a frame header and a payload.
    ///
    /// The header table is used to decode the header list and is updated if necessary.
    ///
    /// # Arguments
    ///
    /// * `header` - The frame header.
    /// * `payload` - The frame payload.
    /// * `header_table` - The header table.
    pub fn deserialize(
        header: &FrameHeader,
        mut payload: Vec<u8>,
        header_table: &mut HeaderTable,
    ) -> Result<Self, Http2Error> {
        // Deserialize the flags from the header.
        let flags: Vec<HeadersFlag> = HeadersFlag::deserialize(header.flags());

        if flags.contains(&HeadersFlag::Padded) {
            let pad_length = payload[0] as usize;

            // Check that the padding length is not 0.
            if pad_length == 0 {
                return Err(Http2Error::FrameError("Padding length is 0".to_string()));
            }
            payload = payload[1..payload.len() - pad_length].to_vec();
        }

        let mut exclusivity: Option<bool> = None;
        let mut stream_dependency: Option<u32> = None;
        let mut weight: Option<u8> = None;

        if flags.contains(&HeadersFlag::Priority) {
            // Check that there is enough space for the priority fields.
            if payload.len() < 5 {
                return Err(Http2Error::FrameError(
                    "Not enough space for priority fields".to_string(),
                ));
            }

            // Parse the priority fields.
            exclusivity = Some(payload[0] & 0b1000_0000 != 0);
            stream_dependency = Some(u32::from_be_bytes([
                payload[0] & 0b0111_1111,
                payload[1],
                payload[2],
                payload[3],
            ]));
            weight = Some(payload[4]);
            payload = payload[5..].to_vec();
        }

        // Try to decode the header list.
        let header_list = HeaderList::decode(&mut payload, header_table)?;

        Ok(Self {
            header_list,
            flags,
            exclusivity,
            stream_dependency,
            weight,
        })
    }
}

impl fmt::Display for Headers {
    /// Format a DATA frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HEADERS Frame\n")?;
        write!(f, "Flags: {:?}\n", self.flags)?;
        write!(f, "Exclusivity: {:?}\n", self.exclusivity)?;
        write!(f, "Stream Dependency: {:?}\n", self.stream_dependency)?;
        write!(f, "Weight: {:?}\n", self.weight)?;
        write!(f, "Header List:\n{}\n", self.header_list)?;

        Ok(())
    }
}