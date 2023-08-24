use std::fmt;

use crate::error::Http2Error;
use crate::frame::{FrameHeader, FramePriority};

/// PRIORITY Frame.
///
/// The PRIORITY frame (type=0x2) specifies the sender-advised priority
/// of a stream (Section 5.3). It can be sent in any stream state,
/// including idle or closed streams.
///
/// +-+-------------------------------------------------------------+
/// |E|                  Stream Dependency (31)                     |
/// +-+-------------+-----------------------------------------------+
/// |   Weight (8)  |
/// +-+-------------+
#[derive(Debug, PartialEq)]

pub struct PriorityFrame {
    stream_id: u32,
    frame_priority: FramePriority,
}

impl PriorityFrame {
    /// Deserialize a PRIORITY frame.
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
                "Expected {} bytes for PRIORITY frame, found {}",
                frame_header.payload_length(),
                bytes.len()
            )));
        }

        Ok(Self {
            stream_id: frame_header.stream_identifier(),
            frame_priority: FramePriority::deserialize(bytes)?,
        })
    }
}

impl fmt::Display for PriorityFrame {
    /// Format a PRIORITY frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PRIORITY\n")?;
        write!(f, "Stream Identifier: {}\n", self.stream_id)?;
        write!(f, "Exclusive: {}\n", self.frame_priority.exclusive())?;
        write!(
            f,
            "Stream Dependency: {}\n",
            self.frame_priority.stream_dependency()
        )?;
        write!(f, "Weight: {}\n", self.frame_priority.weight())
    }
}
