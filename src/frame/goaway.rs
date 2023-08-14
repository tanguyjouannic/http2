use std::fmt;

use crate::{error::Http2Error, frame::FrameHeader};

/// GOAWAY Frame structure.
///
/// The GOAWAY frame (type=0x7) is used to initiate shutdown of a
/// connection or to signal serious error conditions. GOAWAY allows an
/// endpoint to gracefully stop accepting new streams while still
/// finishing processing of previously established streams. This enables
/// administrative actions, like server maintenance.
///
///  +-+-------------------------------------------------------------+
///  |R|                  Last-Stream-ID (31)                        |
///  +-+-------------------------------------------------------------+
///  |                      Error Code (32)                          |
///  +---------------------------------------------------------------+
///  |                  Additional Debug Data (*)                    |
///  +---------------------------------------------------------------+
#[derive(Debug)]
pub struct Goaway {
    reserved: bool,
    last_stream_id: u32,
    error_code: u32,
    additional_debug_data: Option<Vec<u8>>,
}

impl Goaway {
    /// Deserialize a GOAWAY frame from a frame header and a payload.
    ///
    /// # Arguments
    ///
    /// * `header` - The frame header.
    /// * `payload` - The frame payload.
    pub fn deserialize(header: FrameHeader, payload: Vec<u8>) -> Result<Self, Http2Error> {
        // Check if the payload has a correct length.
        if payload.len() < 8 {
            return Err(Http2Error::FrameError(format!(
                "PING frame must be a least 8 bytes. Received {} bytes",
                payload.len()
            )));
        }

        // Extract the reserved bit.
        let reserved = (payload[0] >> 7) != 0;

        // Extract the last stream id.
        let last_stream_id =
            u32::from_be_bytes([payload[0] & 0b0111_1111, payload[1], payload[2], payload[3]]);

        // Extract the error code.
        let error_code = u32::from_be_bytes([payload[4], payload[5], payload[6], payload[7]]);

        // Extract the additional debug data if provided.
        let mut additional_debug_data = None;
        if payload.len() > 8 {
            additional_debug_data = Some(payload[8..].to_vec());
        }

        Ok(Self {
            reserved,
            last_stream_id,
            error_code,
            additional_debug_data,
        })
    }
}

impl fmt::Display for Goaway {
    /// Format a GOAWAY frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GOAWAY Frame\n")?;
        write!(f, "Reserved: {:?}\n", self.reserved)?;
        write!(f, "Last Stream ID: {:?}\n", self.last_stream_id)?;
        write!(f, "Error Code: {:?}\n", self.error_code)?;
        if let Some(additional_debug_data) = &self.additional_debug_data {
            write!(f, "Additional Debug Data: {}\n", String::from_utf8_lossy(additional_debug_data))?;
        }
        Ok(())
    }
}
