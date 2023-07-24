use crate::error::Http2Error;
use crate::header::{HeaderField, HeaderName, HeaderValue};

/// HTTP/2 dynamic header fields table.
pub struct DynamicTable {
    table: Vec<HeaderField>,
    max_size: usize,
    size: usize,
}

impl DynamicTable {
    /// Create a new dynamic table.
    ///
    /// # Arguments
    ///
    /// * `max_size` - The maximum size of the dynamic table.
    pub fn new(max_size: usize) -> DynamicTable {
        DynamicTable {
            table: Vec::new(),
            max_size,
            size: 0,
        }
    }
}

pub struct StaticTable {
    table: Vec<HeaderField>,
}

impl StaticTable {
    /// Create a new static table.
    pub fn new() -> StaticTable {
        let mut table = Vec::new();

        for (name, value) in STATIC_HEADER_FIELDS_TABLE_CONSTANTS.iter() {
            table.push(HeaderField::new(
                HeaderName::new(name.to_string()),
                HeaderValue::new(value.to_string()),
            ));
        }

        StaticTable { table }
    }
}

/// HTTP/2 static header fields table.
pub const STATIC_HEADER_FIELDS_TABLE_CONSTANTS: [(&str, &str); 61] = [
    (":authority", ""),
    (":method", "GET"),
    (":method", "POST"),
    (":path", "/"),
    (":path", "/index.html"),
    (":scheme ", "http"),
    (":scheme", "https"),
    (":status", "200"),
    (":status", "204"),
    (":status", "206"),
    (":status", "304"),
    (":status", "400"),
    (":status", "404"),
    (":status", "500"),
    ("accept-charset", ""),
    ("accept-encoding", "gzip, deflate"),
    ("accept-language", ""),
    ("accept-ranges", ""),
    ("accept", ""),
    ("access-control-allow-origin", ""),
    ("age", ""),
    ("allow", ""),
    ("authorization", ""),
    ("cache-control", ""),
    ("content-disposition", ""),
    ("content-encoding", ""),
    ("content-language", ""),
    ("content-length", ""),
    ("content-location", ""),
    ("content-range", ""),
    ("content-type", ""),
    ("cookie", ""),
    ("date", ""),
    ("etag", ""),
    ("expect", ""),
    ("expires", ""),
    ("from", ""),
    ("host", ""),
    ("if-match", ""),
    ("if-modified-since", ""),
    ("if-none-match", ""),
    ("if-range", ""),
    ("if-unmodified-since", ""),
    ("last-modified", ""),
    ("link", ""),
    ("location", ""),
    ("max-forwards", ""),
    ("proxy-authenticate", ""),
    ("proxy-authorization", ""),
    ("range", ""),
    ("referer", ""),
    ("refresh", ""),
    ("retry-after", ""),
    ("server", ""),
    ("set-cookie", ""),
    ("strict-transport-security", ""),
    ("transfer-encoding", ""),
    ("user-agent", ""),
    ("vary", ""),
    ("via", ""),
    ("www-authenticate", ""),
];
