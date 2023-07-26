use std::fmt;

/// An Error type for the HTTP2 library.
#[derive(Debug)]
pub enum Http2Error {
    FrameError(String),
    NotImplementedError(String),
    HpackError(String),
}

impl fmt::Display for Http2Error {
    /// Display a Http2Error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Http2Error::FrameError(message) => write!(f, "Frame Error: {}", message),
            Http2Error::NotImplementedError(message) => {
                write!(f, "Not Implemented Error: {}", message)
            }
            Http2Error::HpackError(message) => write!(f, "Hpack Error: {}", message),
        }
    }
}

impl std::error::Error for Http2Error {}
