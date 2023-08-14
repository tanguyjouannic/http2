use std::fmt;

use crate::{error::Http2Error, frame::FrameHeader};

/// PING Frame flags.
#[derive(Debug, PartialEq)]
pub enum PingFlag {
    Ack,
}

impl PingFlag {
    /// Deserialize the flags from a byte.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to deserialize the flags from.
    pub fn deserialize(byte: u8) -> Vec<PingFlag> {
        let mut flags: Vec<PingFlag> = Vec::new();

        if byte & 0x1 != 0 {
            flags.push(PingFlag::Ack);
        }

        flags
    }
}

/// PING Frame structure.
///
/// The PING frame (type=0x6) is a mechanism for measuring a minimal
/// round-trip time from the sender, as well as determining whether an
/// idle connection is still functional.  PING frames can be sent from
/// any endpoint.
///
///  +---------------------------------------------------------------+
///  |                                                               |
///  |                      Opaque Data (64)                         |
///  |                                                               |
///  +---------------------------------------------------------------+
#[derive(Debug)]
pub struct Ping {
    flags: Vec<PingFlag>,
    opaque_data: Vec<u8>,
}

impl Ping {
    /// Deserialize a PING frame from a frame header and a payload.
    ///
    /// # Arguments
    ///
    /// * `header` - The frame header.
    /// * `payload` - The frame payload.
    pub fn deserialize(header: &FrameHeader, payload: Vec<u8>) -> Result<Self, Http2Error> {
        // Deserialize the flags.
        let flags = PingFlag::deserialize(header.flags());

        // Check if the payload has the correct length.
        if payload.len() != 8 {
            return Err(Http2Error::FrameError(format!(
                "PING frame payload must be 8 bytes: received {} bytes",
                payload.len()
            )));
        }

        Ok(Self { 
            flags,
            opaque_data: payload 
        })
    }
}

impl fmt::Display for Ping {
    /// Format a PING frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PING Frame\n")?;
        write!(f, "Flags: {:?}\n", self.flags)?;
        write!(f, "Opaque Data: {:?}", self.opaque_data)?;
        Ok(())
    }
}
