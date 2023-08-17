use std::fmt;

use crate::error::Http2Error;
use crate::frame::{FrameHeader, FramePriority};

#[derive(Debug, PartialEq)]

pub struct PriorityFrame {
    stream_id: u32,
    frame_priority: FramePriority,
}

impl PriorityFrame {
    pub fn deserialize(
        frame_header: &FrameHeader,
        bytes: &mut Vec<u8>,
    ) -> Result<Self, Http2Error> {
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
