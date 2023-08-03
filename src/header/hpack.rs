use std::fmt;

use crate::error::Http2Error;
use crate::header::huffman::Tree;

/// A list of HPACK header fields.
#[derive(Clone, Debug, PartialEq)]
pub struct HeaderList {
    header_fields: Vec<HeaderField>,
}

impl HeaderList {
    /// Create a new header list.
    pub fn new(header_fields: Vec<HeaderField>) -> HeaderList {
        HeaderList { header_fields }
    }

    /// Get the header fields of the header list.
    pub fn header_fields(&self) -> &Vec<HeaderField> {
        &self.header_fields
    }

    /// Decode a header list from a byte vector and a header table.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte vector to decode.
    /// * `header_table` - The header table to use.
    pub fn decode(bytes: &mut Vec<u8>, header_table: &mut HeaderTable) -> Result<Self, Http2Error> {
        let mut header_list = HeaderList {
            header_fields: Vec::new(),
        };

        while !bytes.is_empty() {
            let header_field = HeaderField::decode(bytes, header_table)?;
            header_list.header_fields.push(header_field);
        }

        Ok(header_list)
    }
}

impl fmt::Display for HeaderList {
    /// Format a header list.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for header_field in &self.header_fields {
            write!(f, "{}: {}\n", header_field.name(), header_field.value())?;
        }

        Ok(())
    }
}

/// A HTTP/2 header field.
#[derive(Clone, Debug, PartialEq)]
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

    /// Decode a header field from a byte vector and a header table.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte vector.
    /// * `header_table` - The header table.
    pub fn decode(bytes: &mut Vec<u8>, header_table: &mut HeaderTable) -> Result<Self, Http2Error> {
        // Decode the header field representation.
        let representation = HeaderFieldRepresentation::decode(bytes)?;

        // Build the header field from the header field representation.
        Self::from_representation(representation, header_table)
    }

    /// Build a header field from a header field representation and
    /// a header table.
    ///
    /// # Arguments
    ///
    /// * `representation` - The header field representation.
    /// * `header_table` - The header table.
    pub fn from_representation(
        representation: HeaderFieldRepresentation,
        header_table: &mut HeaderTable,
    ) -> Result<Self, Http2Error> {
        match representation {
            HeaderFieldRepresentation::Indexed(index) => {
                let index: usize = match index.value().try_into() {
                    Ok(index) => index,
                    Err(_) => return Err(Http2Error::HpackError("Invalid index".to_string())),
                };

                header_table.get(index)
            }
            HeaderFieldRepresentation::IncrementalIndexingIndexedName(index, value) => {
                let index: usize = match index.value().try_into() {
                    Ok(index) => index,
                    Err(_) => return Err(Http2Error::HpackError("Invalid index".to_string())),
                };

                let indexed_name = match header_table.get(index) {
                    Ok(indexed_name) => indexed_name.name(),
                    Err(error) => return Err(error),
                };

                let name = HeaderName::new(indexed_name);
                let value = HeaderValue::new(value.to_string());

                let header_field = HeaderField::new(name, value);
                header_table.add_entry(header_field.clone());

                Ok(header_field)
            }
            HeaderFieldRepresentation::IncrementalIndexingNewName(name, value) => {
                let name = HeaderName::new(name.to_string());
                let value = HeaderValue::new(value.to_string());

                let header_field = HeaderField::new(name, value);
                header_table.add_entry(header_field.clone());

                Ok(header_field)
            }
            HeaderFieldRepresentation::WithoutIndexingIndexedName(index, value) => {
                let index: usize = match index.value().try_into() {
                    Ok(index) => index,
                    Err(_) => return Err(Http2Error::HpackError("Invalid index".to_string())),
                };

                let indexed_name = match header_table.get(index) {
                    Ok(indexed_name) => indexed_name.name(),
                    Err(error) => return Err(error),
                };

                let name = HeaderName::new(indexed_name);
                let value = HeaderValue::new(value.to_string());

                Ok(HeaderField::new(name, value))
            }
            HeaderFieldRepresentation::WithoutIndexingNewName(name, value) => {
                let name = HeaderName::new(name.to_string());
                let value = HeaderValue::new(value.to_string());

                Ok(HeaderField::new(name, value))
            }
            HeaderFieldRepresentation::NeverIndexedIndexedName(index, value) => {
                let index: usize = match index.value().try_into() {
                    Ok(index) => index,
                    Err(_) => return Err(Http2Error::HpackError("Invalid index".to_string())),
                };

                let indexed_name = match header_table.get(index) {
                    Ok(indexed_name) => indexed_name.name(),
                    Err(error) => return Err(error),
                };

                let name = HeaderName::new(indexed_name);
                let value = HeaderValue::new(value.to_string());

                Ok(HeaderField::new(name, value))
            }
            HeaderFieldRepresentation::NeverIndexedNewName(name, value) => {
                let name = HeaderName::new(name.to_string());
                let value = HeaderValue::new(value.to_string());

                Ok(HeaderField::new(name, value))
            }
            HeaderFieldRepresentation::SizeUpdate(max_size) => {
                header_table.set_max_size(max_size.value() as usize);

                Err(Http2Error::HpackError(
                    "Dynamic Table Size Update is not a header field".to_string(),
                ))
            }
        }
    }
}

impl fmt::Display for HeaderField {
    /// Format a header field.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name.to_string(), self.value.to_string())
    }
}

/// A HTTP/2 header field name.
#[derive(Clone, Debug, PartialEq)]
pub struct HeaderName {
    name: String,
}

impl HeaderName {
    /// Create a new header field name.
    pub fn new(name: String) -> HeaderName {
        HeaderName {
            name: name.to_lowercase(),
        }
    }
}

impl ToString for HeaderName {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl From<&str> for HeaderName {
    fn from(name: &str) -> Self {
        HeaderName::new(name.to_string())
    }
}

/// A HTTP/2 header field value.
#[derive(Clone, Debug, PartialEq)]
pub struct HeaderValue {
    value: String,
}

impl HeaderValue {
    /// Create a new header field value.
    pub fn new(value: String) -> HeaderValue {
        HeaderValue { value }
    }
}

impl ToString for HeaderValue {
    /// Convert the header field value to a String.
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl From<&str> for HeaderValue {
    fn from(name: &str) -> Self {
        HeaderValue::new(name.to_string())
    }
}

pub enum HeaderFieldRepresentation {
    // Indexed Header Field Representation
    //
    // An indexed header field representation identifies an entry in either
    // the static table or the dynamic table.
    //
    // An indexed header field representation causes a header field to be
    // added to the decoded header list.
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | 1 |        Index (7+)         |
    // +---+---------------------------+
    Indexed(HpackInteger),
    // Literal Header Field with Incremental Indexing -- Indexed Name
    //
    // A literal header field with incremental indexing representation
    // results in appending a header field to the decoded header list and
    // inserting it as a new entry into the dynamic table.
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | 0 | 1 |      Index (6+)       |
    // +---+---+-----------------------+
    // | H |     Value Length (7+)     |
    // +---+---------------------------+
    // | Value String (Length octets)  |
    // +-------------------------------+
    IncrementalIndexingIndexedName(HpackInteger, HpackString),
    // Literal Header Field with Incremental Indexing -- New Name
    //
    // A literal header field with incremental indexing representation
    // results in appending a header field to the decoded header list and
    // inserting it as a new entry into the dynamic table.
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | 0 | 1 |           0           |
    // +---+---+-----------------------+
    // | H |     Name Length (7+)      |
    // +---+---------------------------+
    // |  Name String (Length octets)  |
    // +---+---------------------------+
    // | H |     Value Length (7+)     |
    // +---+---------------------------+
    // | Value String (Length octets)  |
    // +-------------------------------+
    IncrementalIndexingNewName(HpackString, HpackString),
    // Literal Header Field without Indexing -- Indexed Name
    //
    // A literal header field without indexing representation results in
    // appending a header field to the decoded header list without altering
    // the dynamic table.
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | 0 | 0 | 0 | 0 |  Index (4+)   |
    // +---+---+-----------------------+
    // | H |     Value Length (7+)     |
    // +---+---------------------------+
    // | Value String (Length octets)  |
    // +-------------------------------+
    WithoutIndexingIndexedName(HpackInteger, HpackString),
    // Literal Header Field without Indexing -- New Name
    //
    // A literal header field without indexing representation results in
    // appending a header field to the decoded header list without altering
    // the dynamic table.
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | 0 | 0 | 0 | 0 |       0       |
    // +---+---+-----------------------+
    // | H |     Name Length (7+)      |
    // +---+---------------------------+
    // |  Name String (Length octets)  |
    // +---+---------------------------+
    // | H |     Value Length (7+)     |
    // +---+---------------------------+
    // | Value String (Length octets)  |
    // +-------------------------------+
    WithoutIndexingNewName(HpackString, HpackString),
    // Literal Header Field Never Indexed -- Indexed Name
    //
    // A literal header field never-indexed representation results in
    // appending a header field to the decoded header list without altering
    // the dynamic table.  Intermediaries MUST use the same representation
    // for encoding this header field.
    //
    //     0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | 0 | 0 | 0 | 1 |  Index (4+)   |
    // +---+---+-----------------------+
    // | H |     Value Length (7+)     |
    // +---+---------------------------+
    // | Value String (Length octets)  |
    // +-------------------------------+
    NeverIndexedIndexedName(HpackInteger, HpackString),
    // Literal Header Field Never Indexed -- New Name
    //
    // A literal header field never-indexed representation results in
    // appending a header field to the decoded header list without altering
    // the dynamic table.  Intermediaries MUST use the same representation
    // for encoding this header field.
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | 0 | 0 | 0 | 1 |       0       |
    // +---+---+-----------------------+
    // | H |     Name Length (7+)      |
    // +---+---------------------------+
    // |  Name String (Length octets)  |
    // +---+---------------------------+
    // | H |     Value Length (7+)     |
    // +---+---------------------------+
    // | Value String (Length octets)  |
    // +-------------------------------+
    NeverIndexedNewName(HpackString, HpackString),
    // Dynamic Table Size Update
    //
    // A dynamic table size update signals a change to the size of the
    // dynamic table.
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | 0 | 0 | 1 |   Max size (5+)   |
    // +---+---------------------------+
    SizeUpdate(HpackInteger),
}

impl HeaderFieldRepresentation {
    pub fn decode(bytes: &mut Vec<u8>) -> Result<HeaderFieldRepresentation, Http2Error> {
        // Check if it is Indexed Header Field Representation.
        if bytes[0] & 0b1000_0000 == 0b1000_0000 {
            let index = HpackInteger::decode(7, bytes)?;
            return Ok(HeaderFieldRepresentation::Indexed(index));
        }

        // Check if it is Literal Header Field with Incremental Indexing.
        if bytes[0] & 0b1100_0000 == 0b0100_0000 {
            // Check if it is Literal Header Field with Incremental Indexing -- Indexed Name.
            if bytes[0] & 0b0011_1111 != 0 {
                let index = HpackInteger::decode(6, bytes)?;
                let value = HpackString::decode(bytes)?;
                return Ok(HeaderFieldRepresentation::IncrementalIndexingIndexedName(
                    index, value,
                ));
            } else {
                *bytes = bytes[1..].to_vec();
                let name = HpackString::decode(bytes)?;
                let value = HpackString::decode(bytes)?;
                return Ok(HeaderFieldRepresentation::IncrementalIndexingNewName(
                    name, value,
                ));
            }
        }

        // Check if it is Literal Header Field without Indexing.
        if bytes[0] & 0b1111_0000 == 0b0000_0000 {
            // Check if it is Literal Header Field without Indexing -- Indexed Name.
            if bytes[0] & 0b0000_1111 != 0 {
                let index = HpackInteger::decode(4, bytes)?;
                let value = HpackString::decode(bytes)?;
                return Ok(HeaderFieldRepresentation::WithoutIndexingIndexedName(
                    index, value,
                ));
            } else {
                *bytes = bytes[1..].to_vec();
                let name = HpackString::decode(bytes)?;
                let value = HpackString::decode(bytes)?;
                return Ok(HeaderFieldRepresentation::WithoutIndexingNewName(
                    name, value,
                ));
            }
        }

        // Check if it is Literal Header Field Never Indexed.
        if bytes[0] & 0b1111_0000 == 0b0001_0000 {
            // Check if it is Literal Header Field Never Indexed -- Indexed Name.
            if bytes[0] & 0b0000_1111 != 0 {
                let index = HpackInteger::decode(4, bytes)?;
                let value = HpackString::decode(bytes)?;
                return Ok(HeaderFieldRepresentation::NeverIndexedIndexedName(
                    index, value,
                ));
            } else {
                *bytes = bytes[1..].to_vec();
                let name = HpackString::decode(bytes)?;
                let value = HpackString::decode(bytes)?;
                return Ok(HeaderFieldRepresentation::NeverIndexedNewName(name, value));
            }
        }

        // Check if it is Dynamic Table Size Update.
        if bytes[0] & 0b1110_0000 == 0b0010_0000 {
            let max_size = HpackInteger::decode(5, bytes)?;
            return Ok(HeaderFieldRepresentation::SizeUpdate(max_size));
        }

        Err(Http2Error::HpackError(
            "Invalid header field representation".to_string(),
        ))
    }
}

/// HPACK header table.
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
    /// Create a new HPACK header table.
    ///
    /// # Arguments
    ///
    /// * `dynamic_table_max_size` - The maximum size of the HPACK dynamic table.
    pub fn new(dynamic_table_max_size: usize) -> HeaderTable {
        HeaderTable {
            static_table: StaticTable::new(),
            dynamic_table: DynamicTable::new(dynamic_table_max_size),
        }
    }

    /// Get a header field from the HPACK header table.
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

    /// Insert a header field into the HPACK header table.
    ///
    /// # Arguments
    ///
    /// * `header_field` - The header field to insert.
    pub fn add_entry(&mut self, header_field: HeaderField) {
        self.dynamic_table.add_entry(header_field);
    }

    /// Set the maximum size of the HPACK dynamic table.
    ///
    /// # Arguments
    ///
    /// * `max_size` - The maximum size of the HPACK dynamic table.
    pub fn set_max_size(&mut self, max_size: usize) {
        self.dynamic_table.set_max_size(max_size);
    }

    /// Get the current size of the HPACK dynamic table.
    pub fn get_dynamic_table_size(&self) -> usize {
        self.dynamic_table.size()
    }
}

/// HPACK dynamic table.
pub struct DynamicTable {
    entries: Vec<HeaderField>,
    size: usize,
    max_size: usize,
}

impl DynamicTable {
    /// Create a new HPACK dynamic table.
    ///
    /// # Arguments
    ///
    /// * `max_size` - The maximum size of the HPACK dynamic table.
    pub fn new(max_size: usize) -> DynamicTable {
        DynamicTable {
            entries: Vec::new(),
            max_size,
            size: 0,
        }
    }

    /// Get the number of entries in the HPACK dynamic table.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Get the size of the HPACK dynamic table.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the maximum size of the HPACK dynamic table.
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Get a header field from the HPACK dynamic table.
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

    /// Update the size of the HPACK dynamic table.
    pub fn update_size(&mut self) {
        self.size = 0;
        for entry in &self.entries {
            self.size += entry.size();
        }
    }

    /// Add a header field to the HPACK dynamic table.
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

    /// Set the maximum size of the HPACK dynamic table.
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

/// HPACK static header fields table.
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

    /// Get the number of header fields of static table.
    pub fn len(&self) -> usize {
        self.table.len()
    }
}

/// Hpack Integer Primitive.
///
/// Integers are used to represent name indexes, header field indexes, or
/// string lengths. An integer representation can start anywhere within
/// an octet. To allow for optimized processing, an integer
/// representation always finishes at the end of an octet.
///
/// An integer is represented in two parts: a prefix that fills the
/// current octet and an optional list of octets that are used if the
/// integer value does not fit within the prefix. The number of bits of
/// the prefix (called N) is a parameter of the integer representation.
///
/// If the integer value is small enough, i.e., strictly less than 2^N-1,
/// it is encoded within the N-bit prefix.
///
///   0   1   2   3   4   5   6   7
/// +---+---+---+---+---+---+---+---+
/// | ? | ? | ? |       Value       |
/// +---+---+---+-------------------+
/// Integer Value Encoded within the Prefix (Shown for N = 5)
///
/// Otherwise, all the bits of the prefix are set to 1, and the value,
/// decreased by 2^N-1, is encoded using a list of one or more octets.
/// The most significant bit of each octet is used as a continuation
/// flag: its value is set to 1 except for the last octet in the list.
/// The remaining bits of the octets are used to encode the decreased
/// value.
///
///   0   1   2   3   4   5   6   7
/// +---+---+---+---+---+---+---+---+
/// | ? | ? | ? | 1   1   1   1   1 |
/// +---+---+---+-------------------+
/// | 1 |    Value-(2^N-1) LSB      |
/// +---+---------------------------+
///                ...
/// +---+---------------------------+
/// | 0 |    Value-(2^N-1) MSB      |
/// +---+---------------------------+
pub struct HpackInteger {
    value: u128,
}

impl HpackInteger {
    /// Create a new Hpack Integer.
    pub fn new(value: u128) -> HpackInteger {
        HpackInteger { value }
    }

    /// Get the value of the Hpack Integer.
    pub fn value(&self) -> u128 {
        self.value
    }

    /// Encode an Hpack Integer.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of bits of the prefix.
    pub fn encode(&self, n: u8) -> Result<Vec<u8>, Http2Error> {
        let mut result: Vec<u8> = Vec::new();

        // Verify that n <= 8 and n != 0.
        if n > 8 || n == 0 {
            return Err(Http2Error::HpackError(
                "Invalid integer prefix size".to_string(),
            ));
        }

        // Compute the max_prefix_value.
        let max_prefix_value = (2u16.pow(n as u32) - 1) as u8;

        // Copy the value.
        let mut integer: u128 = self.value;

        // If the value is smaller than max_prefix_value, encode it on n bits.
        if (integer as u8) < max_prefix_value {
            result.push(integer as u8);
            return Ok(result);
        }

        // Encode the max_prefix_value.
        result.push(max_prefix_value);

        // Substract the max_prefix_value from the value.
        integer -= max_prefix_value as u128;

        // Encode the integer on the required number of octets.
        while integer >= 128 {
            result.push((integer % 128 + 128) as u8);
            integer /= 128;
        }

        result.push(integer as u8);

        Ok(result)
    }

    /// Decode an Hpack Integer.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of bits of the prefix.
    /// * `bytes` - The bytes to decode.
    pub fn decode(n: u8, bytes: &mut Vec<u8>) -> Result<HpackInteger, Http2Error> {
        // Verify that n <= 8 and n != 0.
        if n > 8 || n == 0 {
            return Err(Http2Error::HpackError(
                "Invalid integer prefix size".to_string(),
            ));
        }

        // Compute the maximum prefix value.
        let max_prefix_value = (2u16.pow(n as u32) - 1) as u8;

        // If the first byte is smaller than max_prefix_value, decode it on n bits.
        let masked_prefix = bytes[0] & max_prefix_value;
        if masked_prefix < max_prefix_value {
            let result = HpackInteger::new(masked_prefix as u128);
            match bytes.len() {
                1 => *bytes = Vec::new(),
                _ => *bytes = bytes[1..].to_vec(),
            }
            return Ok(result);
        }

        // Decode the integer on the required number of octets.
        let mut integer: u128 = max_prefix_value as u128;
        let mut multiplier: u8 = 0;

        // Skip the first byte.
        *bytes = bytes[1..].to_vec();

        loop {
            integer = match integer
                .checked_add((bytes[0] & 127) as u128 * 2u128.pow(multiplier as u32))
            {
                Some(integer) => integer,
                None => return Err(Http2Error::HpackError("Integer overflow".to_string())),
            };

            if bytes[0] & 128 != 128 {
                *bytes = bytes[1..].to_vec();
                return Ok(HpackInteger::new(integer));
            } else {
                *bytes = bytes[1..].to_vec();
                multiplier += 7;
            }
        }
    }
}

/// Header field names and header field values can be represented as
/// string literals. A string literal is encoded as a sequence of
/// octets, either by directly encoding the string literal's octets or by
/// using a Huffman code.
///
///   0   1   2   3   4   5   6   7
/// +---+---+---+---+---+---+---+---+
/// | H |    String Length (7+)     |
/// +---+---------------------------+
/// |  String Data (Length octets)  |
/// +-------------------------------+
///
/// A string literal representation contains the following fields:
///
/// H: A one-bit flag, H, indicating whether or not the octets of the
///     string are Huffman encoded.
///
/// String Length: The number of octets used to encode the string
///     literal, encoded as a HPACK integer.
///
/// String Data: The encoded data of the string literal. If H is '0',
///     then the encoded data is the raw octets of the string literal. If
///     H is '1', then the encoded data is the Huffman encoding of the
///     string literal.
pub struct HpackString {
    s: String,
}

impl HpackString {
    pub fn new(s: String) -> HpackString {
        HpackString { s }
    }

    /// Encode an Hpack String.
    ///
    /// # Arguments
    ///
    /// * `huffman_encode` - Whether or not the string should be Huffman encoded.
    pub fn encode(&self, huffman_encode: bool) -> Result<Vec<u8>, Http2Error> {
        let mut result: Vec<u8> = Vec::new();

        // Gather the string's octets.
        let string_octets = self.s.as_bytes();

        // Encode the string if Huffman encoding is required. TODO
        if huffman_encode {}

        // Encode the length of the string.
        let length = HpackInteger::new(string_octets.len() as u128);
        let length_encoded = length.encode(7)?;
        result.extend(length_encoded);

        // Encode the string.
        result.extend(self.s.as_bytes());

        // Add the H bit if the string is Huffman encoded.
        if huffman_encode {
            result[0] |= 0b10000000;
        }

        Ok(result)
    }

    /// Decode an Hpack String.
    ///
    /// The function will delete the bytes that were decoded from the
    /// input bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The bytes to decode.
    pub fn decode(bytes: &mut Vec<u8>) -> Result<HpackString, Http2Error> {
        // Verify that the string is not empty.
        if bytes.len() == 0 {
            return Err(Http2Error::HpackError("Invalid string length".to_string()));
        }

        // Decode the H bit.
        let huffman_encode = bytes[0] & 0b10000000 == 0b10000000;

        // Decode the length of the string.
        let length = HpackInteger::decode(7, bytes)?;
        let length = length.value as usize;

        // Verify that the string is not empty.
        if length == 0 {
            return Err(Http2Error::HpackError("Invalid string length".to_string()));
        }

        // Verify that the string is not too long.
        if bytes.len() < length {
            return Err(Http2Error::HpackError("Invalid string length".to_string()));
        }

        // Gather the string octets.
        let mut string_octets: Vec<u8> = Vec::new();
        for i in 0..length {
            string_octets.push(bytes[i]);
        }

        // Delete the bytes that were decoded.
        *bytes = bytes[length..].to_vec();

        // Decode the string if Huffman encoded. TODO
        if huffman_encode {
            let tree: Tree = Tree::new().unwrap();
            Ok(HpackString::new(tree.decode(&mut string_octets)?))
        } else {
            Ok(HpackString::new(
                String::from_utf8_lossy(&string_octets).into(),
            ))
        }
    }
}

impl ToString for HpackString {
    fn to_string(&self) -> String {
        self.s.clone()
    }
}
