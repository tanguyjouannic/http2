use crate::error::Http2Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Flags {
    EndStream,
    Padded,
    EndHeaders,
    PaddedEndHeaders,
    Priority,
}

///  +-----------------------------------------------+
///  |                 Length (24)                   |
///  +---------------+---------------+---------------+
///  |   Type (8)    |   Flags (8)   |
///  +-+-------------+---------------+-------------------------------+
///  |R|                 Stream Identifier (31)                      |
///  +=+=============================================================+
///  |                   Frame Payload (0...)                      ...
///  +---------------------------------------------------------------+
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
    fn try_from(bytes: &mut Vec<u8>) -> Result<Self, Self::Error> {
        // Check that the stream is long enough to contain a frame header.
        if bytes.len() < 9 {
            return Err(Http2Error::FrameError(
                "Invalid frame header length".to_string(),
            ));
        }

        // Parse the frame header.
        let length = u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]);
        let frame_type: u8 = bytes[3];
        let flags_byte = bytes[4];
        let _reserved = bytes[5] >> 7;
        let stream = u32::from_be_bytes([bytes[5] & 0x7F, bytes[6], bytes[7], bytes[8]]);
        
        // Check that the stream is long enough to get contain the frame.
        if bytes.len() < 9 + length as usize {
            return Err(Http2Error::FrameError(
                "The stream is too short to contain the stream".to_string(),
            ));
        }

        let frame = match frame_type {
            0x0 => Frame::Data(Data::deserialize(bytes[9..9 + length as usize].to_vec(), flags_byte, stream)),
            _ => return Err(Http2Error::FrameError(
                "Invalid frame type".to_string(),
            )),
        };

        // Remove the frame from the bytes.
        *bytes = bytes.split_off(9 + length as usize);

        Ok(frame)
    }
}

impl fmt::Display for Frame {
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

/// DATA Frame structure.
///
/// DATA frames (type=0x0) convey arbitrary, variable-length sequences of
/// octets associated with a stream. One or more DATA frames are used,
/// for instance, to carry HTTP request or response payloads.
///
/// DATA frames MAY also contain padding. Padding can be added to DATA
/// frames to obscure the size of messages. Padding is a security
/// feature
///
///  +---------------+
///  |Pad Length? (8)|
///  +---------------+-----------------------------------------------+
///  |                            Data (*)                         ...
///  +---------------------------------------------------------------+
///  |                           Padding (*)                       ...
///  +---------------------------------------------------------------+
#[derive(Debug)]
pub struct Data {
    pub payload: Vec<u8>,
    pub stream: u32,
    pub flags: Vec<Flags>,
}

impl Data {
    /// Deserialize the flags byte.
    /// 
    /// # Arguments
    /// 
    /// * `byte` - The flags byte.
    fn deserialize_flags(byte: u8) -> Vec<Flags> {
        let mut flags = Vec::new();

        if byte & 0x1 == 0x1 {
            flags.push(Flags::EndStream);
        }

        if byte & 0x8 == 0x8 {
            flags.push(Flags::Padded);
        }

        flags
    }

    /// Deserialize a DATA frame.
    /// 
    /// # Arguments
    /// 
    /// * `frame_payload` - The frame payload.
    /// * `flags_byte` - The flags byte.
    /// * `stream` - The stream identifier.
    pub fn deserialize(frame_payload: Vec<u8>, flags_byte: u8, stream: u32) -> Self {
        let flags = Self::deserialize_flags(flags_byte);

        if flags.contains(&Flags::Padded) {
            let padding_length = frame_payload[0];
            let payload = frame_payload[1..frame_payload.len() - padding_length as usize + 1].to_vec();
            
            Data {
                payload,
                stream,
                flags,
            }
        } else {
            Data {
                payload: frame_payload,
                stream,
                flags,
            }
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Data Frame\n")?;
        write!(f, "  Stream: {}\n", self.stream)?;
        write!(f, "  Flags: {:?}\n", self.flags)?;
        write!(f, "  Payload: {}\n", String::from_utf8_lossy(&self.payload))
    }
}

pub struct Headers {}

pub struct Priority {}

pub struct RstStream {}

pub struct Settings {}

pub struct PushPromise {}

pub struct Ping {}

pub struct GoAway {}

pub struct WindowUpdate {}

pub struct Continuation {}


// pub struct Frame {
//     length: u32,
//     frame_type: FrameType,
//     flags: u8,
//     reserved: u8,
//     stream_identifier: u32,
//     payload: Vec<u8>,
// }

// impl Frame {
//     pub fn from_bytes(bytes: &mut Vec<u8>) -> Result<Self, Http2Error> {
//         // Check that the length of the bytes is 9.
//         if bytes.len() != 9 {
//             return Err(Http2Error::FrameError(
//                 "Invalid frame header length".to_string(),
//             ));
//         }

//         // Try to parse the frame header.
//         let length = u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]);
//         let frame_type = FrameType::try_from(bytes[3])?;
//         let flags = bytes[4];
//         let reserved = bytes[5] >> 7;
//         let stream_identifier = u32::from_be_bytes([bytes[5] & 0x7F, bytes[6], bytes[7], bytes[8]]);
        
//         // Gather the payload with the length
//         let mut payload = Vec::new();
//         for i in 0..length {
//             payload.push(bytes[9 + i as usize]);
//         }

//         // Remove the frame from the bytes.
//         *bytes = bytes.split_off(9 + length as usize);

//         Ok(Frame {
//             length,
//             frame_type,
//             flags,
//             reserved,
//             stream_identifier,
//             payload,
//         })
//     }
// }

// pub enum FrameType {
//     Data,
//     Headers,
//     Priority,
//     RstStream,
//     Settings,
//     PushPromise,
//     Ping,
//     GoAway,
//     WindowUpdate,
//     Continuation,
// }

// impl TryFrom<u8> for FrameType {
//     type Error = Http2Error;

//     /// Try to convert a u8 to a FrameType.
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             0x0 => Ok(FrameType::Data),
//             0x1 => Ok(FrameType::Headers),
//             0x2 => Ok(FrameType::Priority),
//             0x3 => Ok(FrameType::RstStream),
//             0x4 => Ok(FrameType::Settings),
//             0x5 => Ok(FrameType::PushPromise),
//             0x6 => Ok(FrameType::Ping),
//             0x7 => Ok(FrameType::GoAway),
//             0x8 => Ok(FrameType::WindowUpdate),
//             0x9 => Ok(FrameType::Continuation),
//             _ => Err(Http2Error::FrameError("Invalid frame type".to_string())),
//         }
//     }
// }
