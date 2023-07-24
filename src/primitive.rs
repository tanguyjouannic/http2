use std::result;

use crate::error::Http2Error;

/// Http2 Integer Primitive.
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
pub struct Http2Integer {
    value: Vec<u8>,
}

impl Http2Integer {
    pub fn new(value: Vec<u8>) -> Http2Integer {
        Http2Integer { value }
    }

    /// Encode an Http2Integer.
    ///
    /// Pseudocode to represent an integer I is as follows:
    ///
    /// if I < 2^N - 1, encode I on N bits
    /// else
    ///     encode (2^N - 1) on N bits
    ///     I = I - (2^N - 1)
    ///     while I >= 128
    ///         encode (I % 128 + 128) on 8 bits
    ///         I = I / 128
    ///     encode I on 8 bits
    ///
    /// # Arguments
    ///
    /// * `n` - The number of bits of the prefix.
    pub fn encode(&self, n: u8) -> Result<Vec<u8>, Http2Error> {
        // Verify that n < 8.
        if n > 7 {
            return Err(Http2Error::PrimitiveError(
                "Invalid prefix length".to_string(),
            ));
        }

        // Verify that the value is not empty.
        if self.value.is_empty() {
            return Err(Http2Error::PrimitiveError(
                "Http2Integer cannot be empty.".to_string(),
            ));
        }

        // If the value can be encoded on the prefix, encode it.
        if self.value.len() == 1 && self.value[0] < 2u8.pow(n as u32) - 1 {
            return Ok(vec![self.value[0]]);
        }

        // Encode the value using the prefix and the continuation.
        let mut value = self.value.clone();
        value.reverse();

        // Encode the prefix.
        let mut result = Vec::new();
        result.push(2u8.pow(n as u32) - 1);

        loop {
            // Get the first chunk.
            let chunk = value[0] & 0x7F;

            // Shift the values.
            for i in 0..value.len() - 1 {
                println!("i: {}", i);
                value[i] = value[i] >> 7;
                value[i] = value[i] + ((value[i + 1] & 0x7F) << 1);
            }

            *value.last_mut().unwrap() = value.last().unwrap() >> 7;

            // Check if the last chunk is 0.
            if value[value.len() - 1] == 0 {
                value.pop();
            }

            // If this is the last chunk, set the most significant bit to 0.
            if value.len() == 1 {
                if value[0] < 128 {
                    result.push(chunk);
                    break;
                } else {
                    result.push(chunk | 0x80);
                    result.push(1);
                    break;
                }
            } else {
                result.push(chunk | 0x80);
            }
        }

        Ok(result)
    }

    // Decode an Http2Integer.
    //
    // Pseudocode to decode an integer I is as follows:
    //
    // decode I from the next N bits
    // if I < 2^N - 1, return I
    // else
    //     M = 0
    //     repeat
    //         B = next octet
    //         I = I + (B & 127) * 2^M
    //         M = M + 7
    //     while B & 128 == 128
    //     return I
    //
    // # Arguments
    //
    // * `n` - The number of bits of the prefix.
    // * `value` - The value as a list of octets.
    // pub fn decode(n: u8, value: Vec<u8>) -> Result<Self, Http2Error> {}
}
