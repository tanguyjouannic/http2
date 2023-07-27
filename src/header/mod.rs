pub mod hpack;
pub mod huffman;

use crate::error::Http2Error;

/// A HTTP/2 header list.
pub struct HeaderList {
    header_fields: Vec<HeaderField>,
}

impl HeaderList {
    /// Create a new HTTP/2 header list.
    pub fn new() -> HeaderList {
        HeaderList {
            header_fields: Vec::new(),
        }
    }
}

/// A HTTP/2 header field.
pub struct HeaderField {
    name: HeaderName,
    value: HeaderValue,
}

impl HeaderField {
    /// Create a new HTTP/2 header field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the header field.
    /// * `value` - The value of the header field.
    pub fn new(name: HeaderName, value: HeaderValue) -> HeaderField {
        HeaderField { name, value }
    }

    /// Get the name of the header field.
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    /// Get the value of the header field.
    pub fn value(&self) -> String {
        self.value.to_string()
    }

    /// Calculate the size of the header field in octets.
    ///
    /// The size of an entry is the sum of its name's length in octets,
    /// its value's length in octets, and 32.
    pub fn size(&self) -> usize {
        let name_size = self.name.to_string().as_bytes().len();
        let value_size = self.value.to_string().as_bytes().len();

        name_size + value_size + 32
    }
}

/// A HTTP/2 header field name.
pub struct HeaderName {
    name: String,
}

impl ToString for HeaderName {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl HeaderName {
    /// Create a new header field name.
    pub fn new(name: String) -> HeaderName {
        HeaderName {
            name: name.to_lowercase(),
        }
    }

    /// Decode a header field name from a Huffman encoded byte array.
    pub fn from_huffman_encoded(bytes: &[u8]) -> Result<Self, Http2Error> {
        return Err(Http2Error::NotImplementedError(
            "Huffman decoding not yet supported".to_string(),
        ));
    }
}

/// A HTTP/2 header field value.
pub struct HeaderValue {
    value: String,
}

impl ToString for HeaderValue {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl HeaderValue {
    /// Create a new header field value.
    pub fn new(value: String) -> HeaderValue {
        HeaderValue { value }
    }

    /// Decode a header field value from a Huffman encoded byte array.
    pub fn from_huffman_encoded(bytes: &[u8]) -> Result<Self, Http2Error> {
        return Err(Http2Error::NotImplementedError(
            "Huffman decoding not yet supported".to_string(),
        ));
    }
}
