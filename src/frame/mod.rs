pub mod data;
pub mod headers;

use std::fmt;

use crate::error::Http2Error;
use crate::frame::data::Data;
use crate::frame::headers::Headers;

/// HTTP/2 frame header.
///
///  +-----------------------------------------------+
///  |                 Length (24)                   |
///  +---------------+---------------+---------------+
///  |   Type (8)    |   Flags (8)   |
///  +-+-------------+---------------+-------------------------------+
///  |R|                 Stream Identifier (31)                      |
///  +-+-------------------------------------------------------------+
pub struct FrameHeader {
    payload_length: u32,
    frame_type: u8,
    flags: u8,
    reserved: bool,
    stream_identifier: u32,
}

impl FrameHeader {
    /// Get the length of the frame.
    pub fn payload_length(&self) -> u32 {
        self.payload_length
    }

    /// Get the type of the frame.
    pub fn frame_type(&self) -> u8 {
        self.frame_type
    }

    /// Get the flags of the frame.
    pub fn flags(&self) -> u8 {
        self.flags
    }

    /// Get the reserved bit of the frame.
    pub fn reserved(&self) -> bool {
        self.reserved
    }

    /// Get the stream identifier of the frame.
    pub fn stream_identifier(&self) -> u32 {
        self.stream_identifier
    }
}

impl TryFrom<Vec<u8>> for FrameHeader {
    type Error = Http2Error;

    /// Try to extract a frame header from a bytes stream.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The bytes stream to extract the frame header from.
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let payload_length = u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]);
        let frame_type = bytes[3];
        let flags = bytes[4];
        let reserved = (bytes[5] >> 7) != 0;
        let stream_identifier = u32::from_be_bytes([bytes[5] & 0x7F, bytes[6], bytes[7], bytes[8]]);

        Ok(FrameHeader {
            payload_length,
            frame_type,
            flags,
            reserved,
            stream_identifier,
        })
    }
}

/// HTTP/2 frame.
#[derive(Debug)]
pub enum Frame {
    Data(Data),
    // Headers(Headers),
    // Priority(Priority),
    // RstStream(RstStream),
    // Settings(Settings),
    // PushPromise(PushPromise),
    // Ping(Ping),
    // GoAway(GoAway),
    // WindowUpdate(WindowUpdate),
    // Continuation(Continuation),
}

impl TryFrom<&mut Vec<u8>> for Frame {
    type Error = Http2Error;

    /// Try to extract a frame from a bytes stream.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The bytes stream to extract the frame from.
    fn try_from(bytes: &mut Vec<u8>) -> Result<Self, Self::Error> {
        // Retrieve the frame header.
        let header: FrameHeader = bytes[0..9].to_vec().try_into()?;
        *bytes = bytes[9..].to_vec();

        // Get the frame payload.
        let payload = bytes[0..header.payload_length() as usize].to_vec();
        *bytes = bytes[header.payload_length() as usize..].to_vec();

        let frame = match header.frame_type() {
            0x0 => Frame::Data(Data::deserialize(header, payload)),
            // 0x1 => Frame::Headers(Headers::deserialize(header, payload)),
            _ => {
                return Err(Http2Error::FrameError(format!(
                    "Unknown frame type: {}",
                    header.frame_type()
                )))
            }
        };

        Ok(frame)
    }
}

impl fmt::Display for Frame {
    /// Display the Frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Frame::Data(data) => write!(f, "{}", data),
            // Frame::Headers(headers) => write!(f, "{}", headers),
            // Frame::Priority(priority) => write!(f, "{}", priority),
            // Frame::RstStream(rst_stream) => write!(f, "{}", rst_stream),
            // Frame::Settings(settings) => write!(f, "{}", settings),
            // Frame::PushPromise(push_promise) => write!(f, "{}", push_promise),
            // Frame::Ping(ping) => write!(f, "{}", ping),
            // Frame::GoAway(go_away) => write!(f, "{}", go_away),
            // Frame::WindowUpdate(window_update) => write!(f, "{}", window_update),
            // Frame::Continuation(continuation) => write!(f, "{}", continuation),
        }
    }
}

pub struct Priority {}

pub struct RstStream {}

pub struct Settings {}

pub struct PushPromise {}

pub struct Ping {}

pub struct GoAway {}

pub struct WindowUpdate {}

pub struct Continuation {}
