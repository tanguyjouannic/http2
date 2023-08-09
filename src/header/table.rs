use crate::error::Http2Error;
use crate::header::field::HeaderField;
use crate::header::field::{HeaderName, HeaderValue};

/// HTTP/2 HPACK header table.
///
/// The header table contains the union of the static and dynamic tables.
///
/// <----------  Index Address Space ---------->
/// <-- Static  Table -->  <-- Dynamic Table -->
/// +---+-----------+---+  +---+-----------+---+
/// | 1 |    ...    | s |  |s+1|    ...    |s+k|
/// +---+-----------+---+  +---+-----------+---+
///                        ^                   |
///                        |                   V
///                 Insertion Point      Dropping Point
pub struct HeaderTable {
    static_table: StaticTable,
    dynamic_table: DynamicTable,
}

impl HeaderTable {
    /// Create a new header table.
    ///
    /// # Arguments
    ///
    /// * `dynamic_table_max_size` - The maximum size of the dynamic table.
    pub fn new(dynamic_table_max_size: usize) -> HeaderTable {
        HeaderTable {
            static_table: StaticTable::from(STATIC_HEADER_FIELDS_TABLE_CONSTANTS),
            dynamic_table: DynamicTable::new(dynamic_table_max_size),
        }
    }

    /// Get a header field from the header table.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the header field to get.
    pub fn get(&self, index: usize) -> Result<HeaderField, Http2Error> {
        if index <= self.static_table.len() {
            self.static_table.get(index - 1)
        } else {
            self.dynamic_table.get(index - self.static_table.len() - 1)
        }
    }

    /// Insert a header field into the header table.
    ///
    /// # Arguments
    ///
    /// * `header_field` - The header field to insert.
    pub fn add_entry(&mut self, header_field: HeaderField) {
        self.dynamic_table.add_entry(header_field);
    }

    /// Get the index of a header field in the header table.
    ///
    /// # Arguments
    ///
    /// * `header_field` - The header field to search for.
    pub fn contains(&self, header_field: &HeaderField) -> Option<usize> {
        if let Some(index) = self.static_table.contains(header_field) {
            return Some(index + 1);
        };

        if let Some(index) = self.dynamic_table.contains(header_field) {
            return Some(index + self.static_table.len() + 1);
        };

        None
    }

    /// Get the index of a header field that has the same name as
    /// the given header field.
    ///
    /// # Arguments
    ///
    /// * `header_field` - The header field to check.
    ///
    /// # Returns
    ///
    /// * `Some(index)` - The index of the header field in the header table.
    /// * `None` - The header field is not in the header table.
    pub fn contains_name(&self, header_field: &HeaderField) -> Option<usize> {
        if let Some(index) = self.static_table.contains_name(header_field) {
            return Some(index + 1);
        };

        if let Some(index) = self.dynamic_table.contains_name(header_field) {
            return Some(index + self.static_table.len() + 1);
        };

        None
    }

    /// Set the maximum size of the dynamic table.
    ///
    /// # Arguments
    ///
    /// * `max_size` - The maximum size of the dynamic table.
    pub fn set_max_size(&mut self, max_size: usize) {
        self.dynamic_table.set_max_size(max_size);
    }

    /// Get the current size of the dynamic table.
    pub fn get_dynamic_table_size(&self) -> usize {
        self.dynamic_table.size()
    }
}

/// HTTP/2 HPACK dynamic table.
pub struct DynamicTable {
    entries: Vec<HeaderField>,
    size: usize,
    max_size: usize,
}

impl DynamicTable {
    /// Create a new dynamic table.
    ///
    /// # Arguments
    ///
    /// * `max_size` - The maximum size of the dynamic table.
    pub fn new(max_size: usize) -> DynamicTable {
        DynamicTable {
            entries: Vec::new(),
            max_size,
            size: 0,
        }
    }

    /// Get the number of entries in the dynamic table.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Get the size of the dynamic table.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the maximum size of the dynamic table.
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Get a header field from the dynamic table.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the header field to get.
    pub fn get(&self, index: usize) -> Result<HeaderField, Http2Error> {
        match self.entries.get(index) {
            Some(header_field) => Ok(header_field.clone()),
            None => Err(Http2Error::IndexationError(format!(
                "Index {} is out of bounds.",
                index
            ))),
        }
    }

    /// Update the size of the dynamic table.
    pub fn update_size(&mut self) {
        self.size = 0;
        for entry in &self.entries {
            self.size += entry.size();
        }
    }

    /// Check if the dynamic table contains a header field.
    ///
    /// # Arguments
    ///
    /// * `header_field` - The header field to check.
    ///
    /// # Returns
    ///
    /// * `Some(index)` - The index of the header field in the dynamic table.
    /// * `None` - The header field is not in the dynamic table.
    pub fn contains(&self, header_field: &HeaderField) -> Option<usize> {
        for (index, entry) in self.entries.iter().enumerate() {
            if entry == header_field {
                return Some(index);
            }
        }
        None
    }

    /// Check if the dynamic table contains a header field that has the same name as
    /// the given header field.
    ///
    /// # Arguments
    ///
    /// * `header_field` - The header field to check.
    ///
    /// # Returns
    ///
    /// * `Some(index)` - The index of the header field name in the dynamic table.
    /// * `None` - The header field name is not in the dynamic table.
    pub fn contains_name(&self, header_field: &HeaderField) -> Option<usize> {
        for (index, entry) in self.entries.iter().enumerate() {
            if entry.name() == header_field.name() {
                return Some(index);
            }
        }
        None
    }

    /// Add a header field to the dynamic table.
    ///
    /// # Arguments
    ///
    /// * `entry` - The header field to add to the HPACK dynamic table.
    pub fn add_entry(&mut self, entry: HeaderField) {
        // Add the entry at the beginning of the dynamic table.
        self.entries.insert(0, entry);

        // Update the size of the dynamic table.
        self.update_size();

        // Evict entries if the size of the dynamic table is greater than the maximum size.
        while self.size > self.max_size {
            self.entries.pop();
            self.update_size();
        }
    }

    /// Set the maximum size of the dynamic table.
    ///
    /// # Arguments
    ///
    /// * `max_size` - The maximum size of the HPACK dynamic table.
    pub fn set_max_size(&mut self, max_size: usize) {
        // Set the new maximum size of the dynamic table.
        self.max_size = max_size;

        // Evict entries if the size of the dynamic table is greater than the maximum size.
        while self.size > self.max_size {
            self.entries.pop();
            self.update_size();
        }
    }
}

/// HPACK static table constants.
pub const STATIC_HEADER_FIELDS_TABLE_CONSTANTS: [(&str, &str); 61] = [
    (":authority", ""),
    (":method", "GET"),
    (":method", "POST"),
    (":path", "/"),
    (":path", "/index.html"),
    (":scheme", "http"),
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

/// HTTP/2 HPACK static header fields table.
pub struct StaticTable {
    table: Vec<HeaderField>,
}

impl StaticTable {
    /// Get a header field from the static table.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the header field to get.
    pub fn get(&self, index: usize) -> Result<HeaderField, Http2Error> {
        match self.table.get(index) {
            Some(header_field) => Ok(header_field.clone()),
            None => Err(Http2Error::IndexationError(format!(
                "Index {} is out of bounds.",
                index
            ))),
        }
    }

    /// Check if the static table contains a header field.
    ///
    /// # Arguments
    ///
    /// * `header_field` - The header field to check.
    ///
    /// # Returns
    ///
    /// * `Some(index)` - The index of the header field in the static table.
    /// * `None` - The header field is not in the static table.
    pub fn contains(&self, header_field: &HeaderField) -> Option<usize> {
        for (index, entry) in self.table.iter().enumerate() {
            if entry == header_field {
                return Some(index);
            }
        }
        None
    }

    /// Check if the static table contains a header field name.
    ///
    /// # Arguments
    ///
    /// * `header_field` - The header field to check.
    ///
    /// # Returns
    ///
    /// * `Some(index)` - The index of the header field name in the static table.
    /// * `None` - The header field name is not in the static table.
    pub fn contains_name(&self, header_field: &HeaderField) -> Option<usize> {
        for (index, entry) in self.table.iter().enumerate() {
            if entry.name() == header_field.name() {
                return Some(index);
            }
        }
        None
    }

    /// Get the number of header fields of static table.
    pub fn len(&self) -> usize {
        self.table.len()
    }
}

impl From<[(&str, &str); 61]> for StaticTable {
    /// Create a new HTTP/2 HPACK static table.
    ///
    /// # Arguments
    ///
    /// * `constants` - The constants of the HTTP/2 HPACK static table.
    fn from(constants: [(&str, &str); 61]) -> StaticTable {
        let mut table = Vec::new();

        for (name, value) in constants.iter() {
            table.push(HeaderField::new(
                HeaderName::from(*name),
                HeaderValue::from(*value),
            ));
        }

        StaticTable { table }
    }
}
