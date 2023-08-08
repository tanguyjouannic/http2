use std::fmt;

use crate::error::Http2Error;
use crate::header::huffman::Tree;


/// HTTP/2 HPACK Integer Primitive.
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
#[derive(Clone, Debug, PartialEq)]
pub struct HpackInteger {
    value: u128,
}

impl HpackInteger {
    /// Encode a HPACK Integer.
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

    /// Decode a HPACK Integer.
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
            let result = HpackInteger::from(masked_prefix);
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
                return Ok(HpackInteger::from(integer));
            } else {
                *bytes = bytes[1..].to_vec();
                multiplier += 7;
            }
        }
    }
}

impl From<usize> for HpackInteger {
    /// Create a HPACK Integer from a usize.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to build the HPACK Integer from.
    fn from(value: usize) -> Self {
        HpackInteger {
            value: value as u128,
        }
    }
}

impl From<u128> for HpackInteger {
    /// Create a HPACK Integer from a u128.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to build the HPACK Integer from.
    fn from(value: u128) -> Self {
        HpackInteger { value }
    }
}

impl From<u64> for HpackInteger {
    /// Create a HPACK Integer from a u64.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to build the HPACK Integer from.
    fn from(value: u64) -> Self {
        HpackInteger {
            value: value as u128,
        }
    }
}

impl From<u32> for HpackInteger {
    /// Create a HPACK Integer from a u32.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to build the HPACK Integer from.
    fn from(value: u32) -> Self {
        HpackInteger {
            value: value as u128,
        }
    }
}

impl From<u16> for HpackInteger {
    /// Create a HPACK Integer from a u16.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to build the HPACK Integer from.
    fn from(value: u16) -> Self {
        HpackInteger {
            value: value as u128,
        }
    }
}

impl From<u8> for HpackInteger {
    /// Create a HPACK Integer from a u8.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to build the HPACK Integer from.
    fn from(value: u8) -> Self {
        HpackInteger {
            value: value as u128,
        }
    }
}

impl Into<u128> for HpackInteger {
    /// Convert a HPACK Integer into a u128.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a u128.
    fn into(self) -> u128 {
        self.value
    }
}

impl Into<u128> for &HpackInteger {
    /// Convert a reference to a HPACK Integer into a u128.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a u128.
    fn into(self) -> u128 {
        self.value
    }
}

impl Into<u64> for HpackInteger {
    /// Convert a HPACK Integer into a u64.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a u64.
    fn into(self) -> u64 {
        self.value as u64
    }
}

impl Into<u64> for &HpackInteger {
    /// Convert a reference to a HPACK Integer into a u64.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a u64.
    fn into(self) -> u64 {
        self.value as u64
    }
}

impl Into<u32> for HpackInteger {
    /// Convert a HPACK Integer into a u32.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a u32.
    fn into(self) -> u32 {
        self.value as u32
    }
}

impl Into<u32> for &HpackInteger {
    /// Convert a reference to a HPACK Integer into a u32.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a u32.
    fn into(self) -> u32 {
        self.value as u32
    }
}

impl Into<u16> for HpackInteger {
    /// Convert a HPACK Integer into a u16.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a u16.
    fn into(self) -> u16 {
        self.value as u16
    }
}

impl Into<u16> for &HpackInteger {
    /// Convert a reference to a HPACK Integer into a u16.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a u16.
    fn into(self) -> u16 {
        self.value as u16
    }
}

impl Into<u8> for HpackInteger {
    /// Convert a HPACK Integer into a u8.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a u8.
    fn into(self) -> u8 {
        self.value as u8
    }
}

impl Into<u8> for &HpackInteger {
    /// Convert a reference to a HPACK Integer into a u8.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a u8.
    fn into(self) -> u8 {
        self.value as u8
    }
}

impl TryInto<usize> for HpackInteger {
    type Error = Http2Error;

    /// Try to convert a HPACK Integer into a usize.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a usize.
    fn try_into(self) -> Result<usize, Self::Error> {
        match self.value.try_into() {
            Ok(value) => Ok(value),
            Err(_) => Err(Http2Error::HpackError("HPACK Integer overflow".to_string())),
        }
    }
}

impl TryInto<usize> for &HpackInteger {
    type Error = Http2Error;

    /// Try to convert a HPACK Integer into a usize.
    ///
    /// # Returns
    ///
    /// The HPACK Integer as a usize.
    fn try_into(self) -> Result<usize, Self::Error> {
        match self.value.try_into() {
            Ok(value) => Ok(value),
            Err(_) => Err(Http2Error::HpackError("HPACK Integer overflow".to_string())),
        }
    }
}

impl fmt::Display for HpackInteger {
    /// Format a HPACK Integer to be displayed.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// HTTP/2 HPACK String Primitive.
///
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
#[derive(Clone, Debug, PartialEq)]
pub struct HpackString {
    s: String,
}

impl HpackString {
    /// Encode a HPACK String.
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
        let length = HpackInteger::from(string_octets.len() as u128);
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

    /// Decode a HPACK String.
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
            Ok(HpackString::from(tree.decode(&mut string_octets)?))
        } else {
            Ok(HpackString::from(
                String::from_utf8_lossy(&string_octets).to_string(),
            ))
        }
    }
}

impl From<&str> for HpackString {
    /// Create a HPACK String from a &str.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to build the HPACK String from.
    fn from(s: &str) -> Self {
        HpackString { s: s.to_string() }
    }
}

impl From<String> for HpackString {
    /// Create a HPACK String from a String.
    ///
    /// # Arguments
    ///
    /// * `s` - The String to build the HPACK String from.
    fn from(s: String) -> Self {
        HpackString { s }
    }
}

impl Into<String> for HpackString {
    /// Convert a HPACK String into a String.
    ///
    /// # Returns
    ///
    /// The HPACK String as a String.
    fn into(self) -> String {
        self.s
    }
}

impl Into<String> for &HpackString {
    /// Convert a HPACK String into a String.
    ///
    /// # Returns
    ///
    /// The HPACK String as a String.
    fn into(self) -> String {
        self.s.clone()
    }
}

impl fmt::Display for HpackString {
    /// Format a HPACK String to be displayed.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}
