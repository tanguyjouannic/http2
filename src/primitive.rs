use std::result;

use crate::error::Http2Error;

/// HTTP/2 Integer Primitive.
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

    /// Encode an HTTP/2 Integer.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of bits of the prefix.
    pub fn encode(&self, n: u8) -> Result<Vec<u8>, Http2Error> {
        // Verify that n <= 8 and n != 0.
        if n > 8 || n == 0 {
            return Err(Http2Error::PrimitiveError(
                "HTTP/2 Integer prefix must be between 1 and 8 bits.".to_string(),
            ));
        }

        // Verify that the value is not empty.
        if self.value.is_empty() {
            return Err(Http2Error::PrimitiveError(
                "Cannot encode an empty HTTP/2 Integer".to_string(),
            ));
        }

        // Calculate the maximum value that can be encoded on the prefix.
        let max_value = 2u8.pow(n as u32) - 1;

        // If the value can be encoded on the prefix, encode it.
        if self.value.len() == 1 && self.value[0] < max_value {
            return Ok(vec![self.value[0]]);
        }

        // If the value cannot be encoded on the prefix, start the encoding process.

        // Reverse the value to get it in low endian.
        let mut value = self.value.clone();
        value.reverse();

        for v in &value {
            print!("{:b} ", v);
        }
        println!("\n");

        // Subtract the maximum value from the value and take care of the carry.
        for i in 0..value.len() {
            match value[i].checked_sub(max_value) {
                Some(v) => {
                    value[i] = v;
                    break;
                }
                None => {
                    value[i] = value[i].overflowing_sub(max_value).0;
                }
            }
        }

        for v in &value {
            print!("{:b} ", v);
        }

        // Encode the prefix.
        let mut result = Vec::new();
        result.push(max_value);

        loop {
            // Get a chunk.
            let chunk = value[0] & 0x7F;

            // Check if it is the last chunk.
            if value.len() == 1 {
                if value[0] < 128 {
                    result.push(chunk);
                    break;
                } else {
                    result.push(chunk | 0x80);
                    result.push(0x01);
                    break;
                }
            }

            // Shift the values.
            for i in 0..value.len() - 1 {
                value[i] = value[i] >> 7;
                value[i] = value[i] + ((value[i + 1] & 0x7F) << 1);
            }

            *value.last_mut().unwrap() = value.last().unwrap() >> 7;

            // Pop the last value if it is 0.
            if value[value.len() - 1] == 0 {
                value.pop();
            }

            // Push the chunk.
            result.push(chunk | 0x80);
        }

        println!("\n yo");
        for v in &result {
            print!("{:b} ", v);
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
