use std::fmt;

use crate::{error::Http2Error, frame::FrameHeader};

/// PRIORITY Frame structure.
///
/// The PRIORITY frame (type=0x2) specifies the sender-advised priority
/// of a stream (Section 5.3).  It can be sent in any stream state,
/// including idle or closed streams.
///
///  +-+-------------------------------------------------------------+
///  |E|                  Stream Dependency (31)                     |
///  +-+-------------+-----------------------------------------------+
///  |   Weight (8)  |
///  +-+-------------+
#[derive(Debug)]
pub struct Priority {
    header: FrameHeader,
    exclusivity: bool,
    stream_dependency: u32,
    weight: u8,
}

impl Priority {
    /// Deserialize a PRIORITY frame from a frame header and a payload.
    ///
    /// # Arguments
    ///
    /// * `header` - The frame header.
    /// * `payload` - The frame payload.
    pub fn deserialize(header: FrameHeader, payload: Vec<u8>) -> Result<Self, Http2Error> {
        // Check if the payload has the correct length.
        if payload.len() != 5 {
            return Err(Http2Error::FrameError(format!(
                "Invalid payload length for PRIORITY frame: {}",
                payload.len()
            )));
        }

        // Extract the exclusivity.
        let exclusivity = (payload[0] >> 7) != 0;

        // Extract the stream dependency.
        let stream_dependency =
            u32::from_be_bytes([payload[0] & 0b0111_1111, payload[1], payload[2], payload[3]]);

        // Extract the weight.
        let weight = payload[4];

        Ok(Self {
            header,
            exclusivity,
            stream_dependency,
            weight,
        })
    }
}

impl fmt::Display for Priority {
    /// Format a DATA frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PRIORITY Frame\n")?;
        write!(f, "Exclusivity: {:?}\n", self.exclusivity)?;
        write!(f, "Stream Dependency: {:?}\n", self.stream_dependency)?;
        write!(f, "Weight: {:?}\n", self.weight)
    }
}
