use std::fmt;

use crate::error::Http2Error;
use crate::frame::{Frame, FrameFlag, FrameHeader};
use crate::header::list::HeaderList;
use crate::header::table::HeaderTable;

/// CONTINUATION Frame.
///
/// The CONTINUATION frame (type=0x9) is used to continue a sequence of
/// header block fragments. Any number of CONTINUATION frames can be 
/// sent, as long as the preceding frame is on the same stream and is a 
/// HEADERS, PUSH_PROMISE, or CONTINUATION frame without the 
/// END_HEADERS flag set.
///
/// +---------------------------------------------------------------+
/// |                   Header Block Fragment (*)                 ...
/// +---------------------------------------------------------------+
#[derive(Debug, PartialEq)]
pub struct ContinuationFrame {
    end_headers: bool,
    header_list: HeaderList,
}

impl ContinuationFrame {
    /// Deserialize the flags from a byte.
    /// 
    /// # Arguments
    /// 
    /// * `byte` - A byte representing the flags.
    pub fn deserialize_flags(byte: u8) -> Vec<FrameFlag> {
        let mut frame_flags = Vec::new();

        if (byte & 0x04) != 0 {
            frame_flags.push(FrameFlag::EndHeaders);
        }

        frame_flags
    }

    /// Deserialize a CONTINUATION frame.
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
        header_tables: &mut HeaderTable,
    ) -> Result<Self, Http2Error> {
        // Check if the bytes has the right length.
        if bytes.len() != frame_header.payload_length() as usize {
            return Err(Http2Error::FrameError(format!(
                "Expected {} bytes for CONTINUATION frame, found {}",
                frame_header.payload_length(),
                bytes.len()
            )));
        }

        // Deserialize the flags from the header.
        let flags: Vec<FrameFlag> =
            ContinuationFrame::deserialize_flags(frame_header.frame_flags());

        // Retrieve the header list from the payload.
        *bytes = bytes[0..frame_header.payload_length() as usize].to_vec();
        let header_list = HeaderList::decode(bytes, header_tables)?;

        Ok(ContinuationFrame {
            end_headers: flags.contains(&FrameFlag::EndHeaders),
            header_list,
        })
    }
}

impl fmt::Display for ContinuationFrame {
    /// Format a PING frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CONTINUATION\n")?;
        write!(f, "End Headers: {}\n", self.end_headers)?;
        write!(f, "Header List:\n{}", self.header_list)
    }
}

impl Into<Frame> for ContinuationFrame {
    /// Convert the CONTINUATION frame into a generic frame.
    fn into(self) -> Frame {
        Frame::Continuation(self)
    }
}