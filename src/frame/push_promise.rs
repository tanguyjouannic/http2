use std::fmt;

use crate::error::Http2Error;
use crate::frame::{FrameFlag, FrameHeader};
use crate::header::list::HeaderList;
use crate::header::table::HeaderTable;

/// PUSH_PROMISE Frame.
///
/// The PUSH_PROMISE frame (type=0x5) is used to notify the peer endpoint
/// in advance of streams the sender intends to initiate. The
/// PUSH_PROMISE frame includes the unsigned 31-bit identifier of the
/// stream the endpoint plans to create along with a set of headers that
/// provide additional context for the stream.
/// 
/// +---------------+
/// |Pad Length? (8)|
/// +-+-------------+-----------------------------------------------+
/// |R|                  Promised Stream ID (31)                    |
/// +-+-----------------------------+-------------------------------+
/// |                   Header Block Fragment (*)                 ...
/// +---------------------------------------------------------------+
/// |                           Padding (*)                       ...
/// +---------------------------------------------------------------+
#[derive(Debug, PartialEq)]
pub struct PushPromiseFrame {
    stream_id: u32,
    end_headers: bool,
    reserved: bool,
    promised_stream_id: u32,
    header_list: HeaderList,
}

impl PushPromiseFrame {
    /// Deserialize the flags from a byte.
    /// 
    /// # Arguments
    /// 
    /// * `byte` - The byte containing the flags.
    pub fn deserialize_flags(byte: u8) -> Vec<FrameFlag> {
        let mut frame_flags = Vec::new();

        if (byte & 0x04) != 0 {
            frame_flags.push(FrameFlag::EndHeaders);
        }

        if (byte & 0x08) != 0 {
            frame_flags.push(FrameFlag::Padded);
        }

        frame_flags
    }

    /// Deserialize a PUSH_PROMISE frame.
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
                "Expected {} bytes for PUSH_PROMISE frame, found {}",
                frame_header.payload_length(),
                bytes.len()
            )));
        }

        // Deserialize the flags from the header.
        let frame_flags: Vec<FrameFlag> =
            PushPromiseFrame::deserialize_flags(frame_header.frame_flags());

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

        // Deserialize the promise parameters.
        let reserved: bool = (bytes[0] >> 7) != 0;
        let promised_stream_id: u32 =
            u32::from_be_bytes([bytes[0] & 0x7F, bytes[1], bytes[2], bytes[3]]);
        let header_list: HeaderList = HeaderList::decode(&mut bytes[4..].to_vec(), header_table)?;

        Ok(Self {
            stream_id: frame_header.stream_id(),
            end_headers: frame_flags.contains(&FrameFlag::EndHeaders),
            reserved,
            promised_stream_id,
            header_list,
        })
    }
}

impl fmt::Display for PushPromiseFrame {
    /// Format a  PUSH_PROMISE frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PUSH_PROMISE\n")?;
        write!(f, "Stream Identifier: {}\n", self.stream_id)?;
        write!(f, "End Headers: {}\n", self.end_headers)?;
        write!(f, "Reserved: {}\n", self.reserved)?;
        write!(
            f,
            "Promised Stream Identifier: {}\n",
            self.promised_stream_id
        )?;
        write!(f, "Header List:\n{}", self.header_list)
    }
}
