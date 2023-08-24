use std::fmt;

use crate::error::Http2Error;
use crate::frame::{FrameFlag, FrameHeader};

/// PING Frame.
///
/// The PING frame (type=0x6) is a mechanism for measuring a minimal
/// round-trip time from the sender, as well as determining whether an
/// idle connection is still functional. PING frames can be sent from
/// any endpoint.
///
/// +---------------------------------------------------------------+
/// |                                                               |
/// |                      Opaque Data (64)                         |
/// |                                                               |
/// +---------------------------------------------------------------+
#[derive(Debug, PartialEq)]
pub struct PingFrame {
    ack: bool,
    opaque_data: Vec<u8>,
}

impl PingFrame {
    /// Deserialize the flags from a byte.
    /// 
    /// # Arguments
    /// 
    /// * `byte` - The byte containing the flags.
    pub fn deserialize_flags(byte: u8) -> Vec<FrameFlag> {
        let mut frame_flags = Vec::new();

        if (byte & 0x01) != 0 {
            frame_flags.push(FrameFlag::Ack);
        }

        frame_flags
    }

    /// Deserialize a PING frame.
    /// 
    /// The operation is destructive for the bytes vector.
    /// 
    /// # Arguments
    /// 
    /// * `frame_header` - A reference to a FrameHeader.
    /// * `bytes` - A mutable reference to a bytes vector.
    pub fn deserialize(
        frame_header: &FrameHeader,
        bytes: &mut Vec<u8>,
    ) -> Result<Self, Http2Error> {
        // Check if the bytes has the right length.
        if bytes.len() != frame_header.payload_length() as usize {
            return Err(Http2Error::FrameError(format!(
                "Expected {} bytes for PING frame, found {}",
                frame_header.payload_length(),
                bytes.len()
            )));
        }

        // Deserialize the flags from the header.
        let flags: Vec<FrameFlag> = PingFrame::deserialize_flags(frame_header.frame_flags());

        Ok(PingFrame {
            ack: flags.contains(&FrameFlag::Ack),
            opaque_data: bytes[0..8].to_vec(),
        })
    }
}

impl fmt::Display for PingFrame {
    /// Format a PING frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PING\n")?;
        write!(f, "Ack: {}\n", self.ack)?;
        write!(f, "Opaque Data: {:?}\n", self.opaque_data)
    }
}
