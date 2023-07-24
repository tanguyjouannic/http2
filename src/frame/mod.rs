use crate::error::Http2Error;

/// A HTTP/2 frame header.
///
/// All frames begin with a fixed 9-octet header followed by a variable-
/// length payload.
///
///  +-----------------------------------------------+
///  |                 Length (24)                   |
///  +---------------+---------------+---------------+
///  |   Type (8)    |   Flags (8)   |
///  +-+-------------+---------------+-------------------------------+
///  |R|                 Stream Identifier (31)                      |
///  +=+=============================================================+
///  |                   Frame Payload (0...)                      ...
///  +---------------------------------------------------------------+
pub struct FrameHeader {
    length: u32,
    frame_type: FrameType,
    flags: u8,
    reserved: u8,
    stream_identifier: u32,
}

impl FrameHeader {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Http2Error> {
        // Check that the length of the bytes is 9.
        if bytes.len() != 9 {
            return Err(Http2Error::FrameError(
                "Invalid frame header length".to_string(),
            ));
        }

        // Try to parse the frame header.
        let length = u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]);
        let frame_type = FrameType::try_from(bytes[3])?;
        let flags = bytes[4];
        let reserved = bytes[5] >> 7;
        let stream_identifier = u32::from_be_bytes([bytes[5] & 0x7F, bytes[6], bytes[7], bytes[8]]);

        Ok(FrameHeader {
            length,
            frame_type,
            flags,
            reserved,
            stream_identifier,
        })
    }
}

pub enum FrameType {
    Data,
    Headers,
    Priority,
    RstStream,
    Settings,
    PushPromise,
    Ping,
    GoAway,
    WindowUpdate,
    Continuation,
}

impl TryFrom<u8> for FrameType {
    type Error = Http2Error;

    /// Try to convert a u8 to a FrameType.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(FrameType::Data),
            0x1 => Ok(FrameType::Headers),
            0x2 => Ok(FrameType::Priority),
            0x3 => Ok(FrameType::RstStream),
            0x4 => Ok(FrameType::Settings),
            0x5 => Ok(FrameType::PushPromise),
            0x6 => Ok(FrameType::Ping),
            0x7 => Ok(FrameType::GoAway),
            0x8 => Ok(FrameType::WindowUpdate),
            0x9 => Ok(FrameType::Continuation),
            _ => Err(Http2Error::FrameError("Invalid frame type".to_string())),
        }
    }
}
