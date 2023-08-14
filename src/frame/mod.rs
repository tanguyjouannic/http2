pub mod continuation;
pub mod data;
pub mod goaway;
pub mod headers;
pub mod ping;
pub mod priority;
pub mod push_promise;
pub mod rst_stream;
pub mod settings;
pub mod window_update;

use std::fmt;

use crate::error::Http2Error;
use crate::frame::continuation::Continuation;
use crate::frame::data::Data;
use crate::frame::goaway::Goaway;
use crate::frame::headers::Headers;
use crate::frame::ping::Ping;
use crate::frame::priority::Priority;
use crate::frame::push_promise::PushPromise;
use crate::frame::rst_stream::RstStream;
use crate::frame::settings::Settings;
use crate::frame::window_update::WindowUpdate;
use crate::header::table::HeaderTable;

/// HTTP/2 frame header.
///
///  +-----------------------------------------------+
///  |                 Length (24)                   |
///  +---------------+---------------+---------------+
///  |   Type (8)    |   Flags (8)   |
///  +-+-------------+---------------+-------------------------------+
///  |R|                 Stream Identifier (31)                      |
///  +-+-------------------------------------------------------------+
#[derive(Debug)]
pub struct FrameHeader {
    payload_length: u32,
    frame_type: u8,
    flags: u8,
    reserved: bool,
    stream_identifier: u32,
}

impl FrameHeader {
    /// Create a new frame header.
    /// 
    /// # Arguments
    /// 
    /// * `payload_length` - The length of the frame payload.
    /// * `frame_type` - The type of the frame.
    /// * `flags` - The flags of the frame.
    /// * `reserved` - The reserved bit of the frame.
    /// * `stream_identifier` - The stream identifier of the frame.
    pub fn new(
        payload_length: u32,
        frame_type: u8,
        flags: u8,
        reserved: bool,
        stream_identifier: u32,
    ) -> Self {
        Self {
            payload_length,
            frame_type,
            flags,
            reserved,
            stream_identifier,
        }
    }

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

impl TryFrom<&[u8]> for FrameHeader {
    type Error = Http2Error;

    /// Try to extract a frame header from a bytes stream.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The bytes stream to extract the frame header from.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        // Check if the bytes stream is exactly 9 bytes.
        if bytes.len() != 9 {
            return Err(Http2Error::FrameError(format!(
                "Frame header length is not 9: {}",
                bytes.len()
            )));
        }

        // Retrieve the frame header fields.
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
    Headers(Headers),
    Priority(Priority),
    RstStream(RstStream),
    Settings(Settings),
    PushPromise(PushPromise),
    Ping(Ping),
    GoAway(Goaway),
    WindowUpdate(WindowUpdate),
    Continuation(Continuation),
}

impl Frame {
    /// Deserialize a frame based on a frame header and payload.
    ///
    /// The payload has to have a length equal to the length in the frame header.
    /// The header table is updated if necessary.
    ///
    /// # Arguments
    ///
    /// * `frame_header` - The frame header.
    /// * `payload` - The frame payload.
    /// * `header_table` - The header table.
    pub fn deserialize(
        frame_header: &FrameHeader,
        payload: Vec<u8>,
        header_table: &mut HeaderTable,
    ) -> Result<Self, Http2Error> {
        // Deserialize the frame depending on the frame type in the header.
        let frame = match frame_header.frame_type() {
            0x0 => Frame::Data(Data::deserialize(&frame_header, payload)?),
            0x1 => Frame::Headers(Headers::deserialize(&frame_header, payload, header_table)?),
            0x2 => Frame::Priority(Priority::deserialize(&frame_header, payload)?),
            0x3 => Frame::RstStream(RstStream::deserialize(&frame_header, payload)?),
            0x4 => Frame::Settings(Settings::deserialize(&frame_header, payload)?),
            0x5 => Frame::PushPromise(PushPromise::deserialize(&frame_header, payload, header_table)?),
            0x6 => Frame::Ping(Ping::deserialize(&frame_header, payload)?),
            0x7 => Frame::GoAway(Goaway::deserialize(&frame_header, payload)?),
            0x8 => Frame::WindowUpdate(WindowUpdate::deserialize(&frame_header, payload)?),
            0x9 => Frame::Continuation(Continuation::deserialize(&frame_header, payload, header_table)?),
            _ => {
                return Err(Http2Error::FrameError(format!(
                    "Unknown frame type: {}",
                    frame_header.frame_type()
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
            Frame::Headers(headers) => write!(f, "{}", headers),
            Frame::Priority(priority) => write!(f, "{}", priority),
            Frame::RstStream(rst_stream) => write!(f, "{}", rst_stream),
            Frame::Settings(settings) => write!(f, "{}", settings),
            Frame::PushPromise(push_promise) => write!(f, "{}", push_promise),
            Frame::Ping(ping) => write!(f, "{}", ping),
            Frame::GoAway(go_away) => write!(f, "{}", go_away),
            Frame::WindowUpdate(window_update) => write!(f, "{}", window_update),
            Frame::Continuation(continuation) => write!(f, "{}", continuation),
        }
    }
}
