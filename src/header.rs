/// A HTTP/2 header.
pub struct Header {
    name: HeaderName,
    value: HeaderValue,
}

impl Header {
    /// Create a new HTTP/2 header.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the header.
    /// * `value` - The value of the header.
    pub fn new(name: HeaderName, value: HeaderValue) -> Header {
        Header {
            name,
            value,
        }
    }

    /// Get the name of the header.
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    /// Get the value of the header.
    pub fn value(&self) -> String {
        self.value.to_string()
    }
}

/// A HTTP/2 header name.
pub struct HeaderName {
    name: String,
}

impl ToString for HeaderName {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

/// A HTTP/2 header value.
pub struct HeaderValue {
    value: String,
}

impl ToString for HeaderValue {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}