use std::fmt;

use crate::error::Http2Error;
use crate::frame::{FrameFlag, FrameHeader};
use crate::header::list::HeaderList;
use crate::header::table::HeaderTable;

#[derive(Debug, PartialEq)]
pub struct ContinuationFrame {
    end_headers: bool,
    header_list: HeaderList,
}

impl ContinuationFrame {
    pub fn deserialize_flags(byte: u8) -> Vec<FrameFlag> {
        let mut frame_flags = Vec::new();

        if (byte & 0x04) != 0 {
            frame_flags.push(FrameFlag::EndHeaders);
        }

        frame_flags
    }

    pub fn deserialize(
        frame_header: &FrameHeader,
        bytes: &mut Vec<u8>,
        header_tables: &mut HeaderTable,
    ) -> Result<Self, Http2Error> {
        // Check if the bytes vector contains enough bytes.
        if bytes.len() < frame_header.payload_length() as usize {
            return Err(Http2Error::NotEnoughBytes(format!(
                "CONTINUATION frame needed {} bytes, found {}",
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
