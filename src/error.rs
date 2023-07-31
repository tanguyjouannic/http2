use std::fmt;

/// An Error type for the HTTP2 library.
#[derive(Debug)]
pub enum Http2Error {
    FrameError(String),
    HpackError(String),
    HuffmanDecodingError(String),
    HeaderError(String),
    IndexationError(String),
}

impl fmt::Display for Http2Error {
    /// Display a Http2Error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Http2Error::FrameError(message) => write!(f, "Frame Error: {}", message),
            Http2Error::HpackError(message) => write!(f, "Hpack Error: {}", message),
            Http2Error::HuffmanDecodingError(message) => {
                write!(f, "Huffman Decoding Error: {}", message)
            }
            Http2Error::HeaderError(message) => {
                write!(f, "Invalid Header Error: {}", message)
            }
            Http2Error::IndexationError(message) => {
                write!(f, "Indexation Error: {}", message)
            }
        }
    }
}

impl std::error::Error for Http2Error {}
