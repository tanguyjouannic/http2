use std::fmt;

use crate::{error::Http2Error, frame::FrameHeader, header::{list::HeaderList, table::HeaderTable}};

/// PUSH_PROMISE Frame flags.
#[derive(Debug, PartialEq)]
pub enum PushPromiseFlag {
    EndHeaders,
    Padded,
}

impl PushPromiseFlag {
    /// Parse the flags from a byte.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to extract the flags from.
    pub fn parse_flags(byte: u8) -> Vec<PushPromiseFlag> {
        let mut flags: Vec<PushPromiseFlag> = Vec::new();

        if byte & 0x4 != 0 {
            flags.push(PushPromiseFlag::EndHeaders);
        }

        if byte & 0x8 != 0 {
            flags.push(PushPromiseFlag::Padded);
        }

        flags
    }
}

/// PUSH_PROMISE Frame structure.
///
/// The PUSH_PROMISE frame (type=0x5) is used to notify the peer endpoint
/// in advance of streams the sender intends to initiate. The
/// PUSH_PROMISE frame includes the unsigned 31-bit identifier of the
/// stream the endpoint plans to create along with a set of headers that
/// provide additional context for the stream.
/// 
///  +---------------+
///  |Pad Length? (8)|
///  +-+-------------+-----------------------------------------------+
///  |R|                  Promised Stream ID (31)                    |
///  +-+-----------------------------+-------------------------------+
///  |                   Header Block Fragment (*)                 ...
///  +---------------------------------------------------------------+
///  |                           Padding (*)                       ...
///  +---------------------------------------------------------------+
#[derive(Debug)]
pub struct PushPromise {
    header: FrameHeader,
    parsed_flags: Vec<PushPromiseFlag>,
    reserved: bool,
    promised_stream_id: u32,
    header_list: HeaderList,
}

impl PushPromise {
    /// Deserialize a PRIORITY frame from a frame header and a payload.
    ///
    /// The header table is used to decode the header list and is updated if necessary.
    ///
    /// # Arguments
    ///
    /// * `header` - The frame header.
    /// * `payload` - The frame payload.
    /// * `header_table` - The header table.
    pub fn deserialize(
        header: FrameHeader,
        mut payload: Vec<u8>,
        header_table: &mut HeaderTable,
    ) -> Result<Self, Http2Error> {
        // Check that the payload is at least 4 bytes long.
        if payload.len() < 4 {
            return Err(Http2Error::FrameError(
                "Payload length must be more than 4 bytes".to_string(),
            ));
        }

        // Parse the flags from the header.
        let parsed_flags: Vec<PushPromiseFlag> = PushPromiseFlag::parse_flags(header.flags());

        if parsed_flags.contains(&PushPromiseFlag::Padded) {
            let pad_length = payload[0] as usize;

            // Check that the padding length is not 0.
            if pad_length == 0 {
                return Err(Http2Error::FrameError("Padding length is 0".to_string()));
            }
            payload = payload[1..payload.len() - pad_length].to_vec();
        }

        // Retrieve the reserved bit and the promised stream id.
        let reserved: bool = payload[0] & 0b1000_0000 != 0;
        let promised_stream_id: u32 = u32::from_be_bytes([
            payload[0] & 0b0111_1111,
            payload[1],
            payload[2],
            payload[3],
        ]);

        // Decode the header list.
        let header_list = HeaderList::decode(&mut payload[4..].to_vec(), header_table)?;

        Ok(Self {
            header,
            parsed_flags,
            reserved,
            promised_stream_id,
            header_list,
        })
    }
}

impl fmt::Display for PushPromise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PUSH_PROMISE Frame\n")?;
        write!(f, "Parsed Flags: {:?}\n", self.parsed_flags)?;
        write!(f, "Reserved: {:?}\n", self.reserved)?;
        write!(f, "Promised Stream ID: {:?}\n", self.promised_stream_id)?;
        write!(f, "Header List:\n{}\n", self.header_list)
    }
}