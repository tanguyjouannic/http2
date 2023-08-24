mod continuation;
mod data;
mod go_away;
mod headers;
mod ping;
mod priority;
mod push_promise;
mod rst_stream;
mod settings;
mod window_update;

use std::fmt;

use crate::error::Http2Error;
use crate::frame::{
    continuation::ContinuationFrame, data::DataFrame, go_away::GoAwayFrame, headers::HeadersFrame,
    ping::PingFrame, priority::PriorityFrame, push_promise::PushPromiseFrame,
    rst_stream::RstStreamFrame, settings::SettingsFrame, window_update::WindowUpdateFrame,
};
use crate::header::table::HeaderTable;

/// HTTP/2 frame.
/// 
/// +-----------------------------------------------+
/// |                 Length (24)                   |
/// +---------------+---------------+---------------+
/// |   Type (8)    |   Flags (8)   |
/// +-+-------------+---------------+-------------------------------+
/// |R|                 Stream Identifier (31)                      |
/// +=+=============================================================+
/// |                   Frame Payload (0...)                      ...
/// +---------------------------------------------------------------+
#[derive(Debug, PartialEq)]
pub enum Frame {
    Data(DataFrame),
    Headers(HeadersFrame),
    Priority(PriorityFrame),
    RstStream(RstStreamFrame),
    Settings(SettingsFrame),
    PushPromise(PushPromiseFrame),
    Ping(PingFrame),
    GoAway(GoAwayFrame),
    WindowUpdate(WindowUpdateFrame),
    Continuation(ContinuationFrame),
}

impl Frame {
    /// Deserialize a Frame.
    /// 
    /// # Arguments
    /// 
    /// * `stream` - A mutable reference to a bytes vector.
    /// * `header_table` - A mutable reference to a HeaderTable.
    pub fn deserialize(
        stream: &mut Vec<u8>,
        header_table: &mut HeaderTable,
    ) -> Result<Frame, Http2Error> {
        // Make a copy of the bytes vector.
        let mut bytes: Vec<u8> = stream.clone();

        // Try to extract the frame header from the bytes stream.
        let frame_header = FrameHeader::deserialize(&mut bytes)?;

        // Verify that there is enough bytes to deserialize the payload.
        if bytes.len() < frame_header.payload_length() as usize {
            return Err(Http2Error::NotEnoughBytes(format!(
                "Frame payload needs at least {} bytes, found {}",
                frame_header.payload_length(),
                bytes.len(),
            )));
        }

        // Retrieve only the payload bytes.
        bytes = bytes[..frame_header.payload_length() as usize].to_vec();

        // Deserialize the frame.
        let frame = match frame_header.frame_type() {
            0x00 => Frame::Data(DataFrame::deserialize(&frame_header, &mut bytes)?),
            0x01 => Frame::Headers(HeadersFrame::deserialize(
                &frame_header,
                &mut bytes,
                header_table,
            )?),
            0x02 => Frame::Priority(PriorityFrame::deserialize(&frame_header, &mut bytes)?),
            0x03 => Frame::RstStream(RstStreamFrame::deserialize(&frame_header, &mut bytes)?),
            0x04 => Frame::Settings(SettingsFrame::deserialize(&frame_header, &mut bytes)?),
            0x05 => Frame::PushPromise(PushPromiseFrame::deserialize(
                &frame_header,
                &mut bytes,
                header_table,
            )?),
            0x06 => Frame::Ping(PingFrame::deserialize(&frame_header, &mut bytes)?),
            0x07 => Frame::GoAway(GoAwayFrame::deserialize(&frame_header, &mut bytes)?),
            0x08 => Frame::WindowUpdate(WindowUpdateFrame::deserialize(&frame_header, &mut bytes)?),
            0x09 => Frame::Continuation(ContinuationFrame::deserialize(
                &frame_header,
                &mut bytes,
                header_table,
            )?),
            _ => {
                return Err(Http2Error::FrameError(format!(
                    "Could not deserialize Frame: unknown frame type {}",
                    frame_header.frame_type()
                )))
            }
        };

        // Remove the frame from the bytes stream.
        *stream = stream[9 + frame_header.payload_length() as usize..].to_vec();

        Ok(frame)
    }
}

impl fmt::Display for Frame {
    /// Format any Frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Frame::Data(frame) => write!(f, "{}", frame),
            Frame::Headers(frame) => write!(f, "{}", frame),
            Frame::Priority(frame) => write!(f, "{}", frame),
            Frame::RstStream(frame) => write!(f, "{}", frame),
            Frame::Settings(frame) => write!(f, "{}", frame),
            Frame::PushPromise(frame) => write!(f, "{}", frame),
            Frame::Ping(frame) => write!(f, "{}", frame),
            Frame::GoAway(frame) => write!(f, "{}", frame),
            Frame::WindowUpdate(frame) => write!(f, "{}", frame),
            Frame::Continuation(frame) => write!(f, "{}", frame),
        }
    }
}

/// HTTP/2 frame header.
///
/// +-----------------------------------------------+
/// |                 Length (24)                   |
/// +---------------+---------------+---------------+
/// |   Type (8)    |   Flags (8)   |
/// +-+-------------+---------------+-------------------------------+
/// |R|                 Stream Identifier (31)                      |
/// +-+-------------------------------------------------------------+
#[derive(Debug, PartialEq)]
pub struct FrameHeader {
    payload_length: u32,
    frame_type: u8,
    frame_flags: u8,
    reserved: bool,
    stream_identifier: u32,
}

impl FrameHeader {
    /// Deserialize a FrameHeader.
    /// 
    /// If the deserialization is successful, the FrameHeader is removed from the bytes vector.
    /// 
    /// # Arguments
    /// 
    /// * `bytes` - A mutable reference to a bytes vector.
    pub fn deserialize(bytes: &mut Vec<u8>) -> Result<Self, Http2Error> {
        // Check if the bytes stream has at least 9 bytes.
        if bytes.len() < 9 {
            return Err(Http2Error::NotEnoughBytes(format!(
                "Frame header needs at least 9 bytes, found {}",
                bytes.len()
            )));
        }

        // Retrieve the frame header fields.
        let payload_length = u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]);
        let frame_type = bytes[3];
        let frame_flags = bytes[4];
        let reserved = (bytes[5] >> 7) != 0;
        let stream_identifier = u32::from_be_bytes([bytes[5] & 0x7F, bytes[6], bytes[7], bytes[8]]);

        // Remove the frame header from the bytes stream.
        *bytes = bytes[9..].to_vec();

        Ok(FrameHeader {
            payload_length,
            frame_type,
            frame_flags,
            reserved,
            stream_identifier,
        })
    }

    pub fn payload_length(&self) -> u32 {
        self.payload_length
    }

    pub fn frame_type(&self) -> u8 {
        self.frame_type
    }

    pub fn frame_flags(&self) -> u8 {
        self.frame_flags
    }

    pub fn reserved(&self) -> bool {
        self.reserved
    }

    pub fn stream_identifier(&self) -> u32 {
        self.stream_identifier
    }
}

/// HTTP/2 frame flags.
#[derive(Debug, PartialEq)]
pub enum FrameFlag {
    EndStream,
    Padded,
    EndHeaders,
    Priority,
    Ack,
}

/// HTTP/2 frame priority.
#[derive(Debug, PartialEq)]
pub struct FramePriority {
    exclusive: bool,
    stream_dependency: u32,
    weight: u8,
}

impl FramePriority {
    /// Deserialize a FramePriority.
    /// 
    /// If the deserialization is successful, the FramePriority is removed from the bytes vector.
    /// 
    /// # Arguments
    /// 
    /// * `bytes` - A mutable reference to a bytes vector.
    pub fn deserialize(bytes: &mut Vec<u8>) -> Result<Self, Http2Error> {
        // Check if the bytes stream has at least 5 bytes.
        if bytes.len() < 5 {
            return Err(Http2Error::NotEnoughBytes(format!(
                "Frame priority needs at least 5 bytes, found {}",
                bytes.len()
            )));
        }

        // Retrieve the frame priority fields.
        let exclusive = (bytes[0] >> 7) != 0;
        let stream_dependency = u32::from_be_bytes([bytes[0] & 0x7F, bytes[1], bytes[2], bytes[3]]);
        let weight = bytes[4];

        // Remove the frame priority from the bytes stream.
        *bytes = bytes[5..].to_vec();

        Ok(FramePriority {
            exclusive,
            stream_dependency,
            weight,
        })
    }

    pub fn exclusive(&self) -> bool {
        self.exclusive
    }

    pub fn stream_dependency(&self) -> u32 {
        self.stream_dependency
    }

    pub fn weight(&self) -> u8 {
        self.weight
    }
}

impl fmt::Display for FramePriority {
    /// Format a Frame priority.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Exclusive: {}\n", self.exclusive)?;
        write!(f, "Stream Dependency: {}\n", self.stream_dependency)?;
        write!(f, "Weight: {}\n", self.weight)
    }
}
