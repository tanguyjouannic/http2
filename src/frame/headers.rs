use std::fmt;

use crate::error::Http2Error;
use crate::frame::{FrameFlag, FrameHeader, FramePriority};
use crate::header::list::HeaderList;
use crate::header::table::HeaderTable;

/// HEADERS Frame.
///
/// The HEADERS frame (type=0x1) is used to open a stream (Section 5.1),
/// and additionally carries a header block fragment. HEADERS frames can
/// be sent on a stream in the "idle", "reserved (local)", "open", or
/// "half-closed (remote)" state.
///
/// +---------------+
/// |Pad Length? (8)|
/// +-+-------------+-----------------------------------------------+
/// |E|                 Stream Dependency? (31)                     |
/// +-+-------------+-----------------------------------------------+
/// |  Weight? (8)  |
/// +-+-------------+-----------------------------------------------+
/// |                   Header Block Fragment (*)                 ...
/// +---------------------------------------------------------------+
/// |                           Padding (*)                       ...
/// +---------------------------------------------------------------+
#[derive(Debug, PartialEq)]

pub struct HeadersFrame {
    stream_id: u32,
    end_stream: bool,
    end_headers: bool,
    frame_priority: Option<FramePriority>,
    header_list: HeaderList,
}

impl HeadersFrame {
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

        if (byte & 0x04) != 0 {
            frame_flags.push(FrameFlag::EndHeaders);
        }

        if (byte & 0x08) != 0 {
            frame_flags.push(FrameFlag::Padded);
        }

        if (byte & 0x20) != 0 {
            frame_flags.push(FrameFlag::Priority);
        }

        frame_flags
    }

    /// Deserialize a HEADERS frame.
    /// 
    /// The operation is destructive for the bytes vector.
    /// 
    /// # Arguments
    /// 
    /// * `frame_header` - A reference to a FrameHeader.
    /// * `bytes` - A mutable reference to a bytes vector.
    /// * `header_tables` - A mutable reference to a HeaderTable.
    pub fn deserialize(
        frame_header: &FrameHeader,
        bytes: &mut Vec<u8>,
        header_table: &mut HeaderTable,
    ) -> Result<Self, Http2Error> {
        // Check if the bytes has the right length.
        if bytes.len() != frame_header.payload_length() as usize {
            return Err(Http2Error::FrameError(format!(
                "Expected {} bytes for HEADERS frame, found {}",
                frame_header.payload_length(),
                bytes.len()
            )));
        }

        // Deserialize the flags from the header.
        let frame_flags: Vec<FrameFlag> =
            HeadersFrame::deserialize_flags(frame_header.frame_flags());

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

        // Handle the priority if needed.
        let mut frame_priority: Option<FramePriority> = None;
        if frame_flags.contains(&FrameFlag::Priority) {
            frame_priority = Some(FramePriority::deserialize(bytes)?);
        }

        // Decode the header list (the header table is updated).
        let header_list = HeaderList::decode(bytes, header_table)?;

        Ok(Self {
            stream_id: frame_header.stream_identifier(),
            end_stream: frame_flags.contains(&FrameFlag::EndStream),
            end_headers: frame_flags.contains(&FrameFlag::EndHeaders),
            frame_priority,
            header_list,
        })
    }
}

impl fmt::Display for HeadersFrame {
    /// Format a HEADERS frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HEADERS\n")?;
        write!(f, "Stream Identifier: {}\n", self.stream_id)?;
        write!(f, "End Stream: {}\n", self.end_stream)?;
        write!(f, "End Headers: {}\n", self.end_headers)?;
        if let Some(frame_priority) = &self.frame_priority {
            write!(f, "{}", frame_priority)?;
        }
        write!(f, "Header List:\n{}\n", self.header_list)
    }
}
