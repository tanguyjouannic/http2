use crate::error::Http2Error;
use crate::header::{HeaderField, HeaderName, HeaderValue};
use std::result;

/// HPACK dynamic header fields table.
pub struct DynamicTable {
    table: Vec<HeaderField>,
    max_size: usize,
    size: usize,
}

impl DynamicTable {
    /// Create a new HPACK dynamic table.
    ///
    /// # Arguments
    ///
    /// * `max_size` - The maximum size of the HPACK dynamic table.
    pub fn new(max_size: usize) -> DynamicTable {
        DynamicTable {
            table: Vec::new(),
            max_size,
            size: 0,
        }
    }
}

/// HPACK static header fields constants.
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
        if huffman_encode {
            return Err(Http2Error::NotImplementedError(
                "Huffman encoding not implemented".to_string(),
            ));
        }

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

        // Decode the string if Huffman encoded. TODO
        if huffman_encode {
            return Err(Http2Error::NotImplementedError(
                "Huffman encoding not implemented".to_string(),
            ));
        }

        Ok(HpackString::new(
            String::from_utf8_lossy(&string_octets).into(),
        ))
    }
}
