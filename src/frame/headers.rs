use std::fmt;

use crate::{frame::FrameHeader, header::list::HeaderList};

/// HEADERS Frame flags.
#[derive(Debug)]
pub enum HeadersFlag {
    EndStream,
    EndHeaders,
    Padded,
    Priority,
}

/// HEADERS Frame structure.
/// 
/// The HEADERS frame (type=0x1) is used to open a stream (Section 5.1),
/// and additionally carries a header block fragment.  HEADERS frames can
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
pub struct Headers {
    header_list: HeaderList,
    stream: u32,
    flags: Vec<HeadersFlag>,
}

impl Headers {
    pub fn deserialize(header: FrameHeader, payload: Vec<u8>) -> Self {
        let mut flags = Vec::new();

        if header.flags() & 0x1 != 0 {
            flags.push(HeadersFlag::EndStream);
        }

        if header.flags() & 0x4 != 0 {
            flags.push(HeadersFlag::EndHeaders);
        }

        if header.flags() & 0x8 != 0 {
            flags.push(HeadersFlag::Padded);
        }

        if header.flags() & 0x20 != 0 {
            flags.push(HeadersFlag::Priority);
        }

        let header_list = HeaderList::deserialize(payload);

        Self {
            header_list,
            stream: header.stream_identifier(),
            flags,
        }
    }
}

impl fmt::Display for Headers {
    /// Format a HEADERS frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Headers Frame\n")?;
        write!(f, "Stream Identifier: {}\n", self.stream)?;
        write!(f, "Flags: {:?}\n", self.flags)?;
        write!(f, "Header List: {}", self.header_list)
    }
}
