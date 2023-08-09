use crate::error::Http2Error;
use crate::header::primitive::{HpackInteger, HpackString};

/// HTTP/2 HPACK header field representation.
pub enum HeaderRepresentation {
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

impl HeaderRepresentation {
    pub fn decode(bytes: &mut Vec<u8>) -> Result<HeaderRepresentation, Http2Error> {
        // Check if it is Indexed Header Field Representation.
        if bytes[0] & 0b1000_0000 == 0b1000_0000 {
            let index = HpackInteger::decode(7, bytes)?;
            return Ok(HeaderRepresentation::Indexed(index));
        }

        // Check if it is Literal Header Field with Incremental Indexing.
        if bytes[0] & 0b1100_0000 == 0b0100_0000 {
            // Check if it is Literal Header Field with Incremental Indexing -- Indexed Name.
            if bytes[0] & 0b0011_1111 != 0 {
                let index = HpackInteger::decode(6, bytes)?;
                let value = HpackString::decode(bytes)?;
                return Ok(HeaderRepresentation::IncrementalIndexingIndexedName(
                    index, value,
                ));
            } else {
                *bytes = bytes[1..].to_vec();
                let name = HpackString::decode(bytes)?;
                let value = HpackString::decode(bytes)?;
                return Ok(HeaderRepresentation::IncrementalIndexingNewName(
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
                return Ok(HeaderRepresentation::WithoutIndexingIndexedName(
                    index, value,
                ));
            } else {
                *bytes = bytes[1..].to_vec();
                let name = HpackString::decode(bytes)?;
                let value = HpackString::decode(bytes)?;
                return Ok(HeaderRepresentation::WithoutIndexingNewName(name, value));
            }
        }

        // Check if it is Literal Header Field Never Indexed.
        if bytes[0] & 0b1111_0000 == 0b0001_0000 {
            // Check if it is Literal Header Field Never Indexed -- Indexed Name.
            if bytes[0] & 0b0000_1111 != 0 {
                let index = HpackInteger::decode(4, bytes)?;
                let value = HpackString::decode(bytes)?;
                return Ok(HeaderRepresentation::NeverIndexedIndexedName(index, value));
            } else {
                *bytes = bytes[1..].to_vec();
                let name = HpackString::decode(bytes)?;
                let value = HpackString::decode(bytes)?;
                return Ok(HeaderRepresentation::NeverIndexedNewName(name, value));
            }
        }

        // Check if it is Dynamic Table Size Update.
        if bytes[0] & 0b1110_0000 == 0b0010_0000 {
            let max_size = HpackInteger::decode(5, bytes)?;
            return Ok(HeaderRepresentation::SizeUpdate(max_size));
        }

        Err(Http2Error::HpackError(
            "Invalid header field representation".to_string(),
        ))
    }

    /// Encodes the header field representation into a byte vector.
    pub fn encode(&self, huffman_encode_name: bool, huffman_encode_value: bool) -> Vec<u8> {
        match self {
            HeaderRepresentation::Indexed(index) => {
                let mut bytes = index.encode(7).unwrap();
                bytes[0] |= 0b1000_0000;
                bytes
            }
            HeaderRepresentation::IncrementalIndexingIndexedName(index, value) => {
                let mut bytes = index.encode(6).unwrap();
                bytes[0] |= 0b0100_0000;
                bytes.append(&mut value.encode(huffman_encode_value).unwrap());
                bytes
            }
            HeaderRepresentation::IncrementalIndexingNewName(name, value) => {
                let mut bytes: Vec<u8> = Vec::new();
                bytes.push(0b0100_0000);
                bytes.append(&mut name.encode(huffman_encode_name).unwrap());
                bytes.append(&mut value.encode(huffman_encode_value).unwrap());
                bytes
            }
            HeaderRepresentation::WithoutIndexingIndexedName(index, value) => {
                let mut bytes = index.encode(4).unwrap();
                bytes[0] |= 0b0000_0000;
                bytes.append(&mut value.encode(huffman_encode_value).unwrap());
                bytes
            }
            HeaderRepresentation::WithoutIndexingNewName(name, value) => {
                let mut bytes: Vec<u8> = Vec::new();
                bytes.push(0b0000_0000);
                bytes.append(&mut name.encode(huffman_encode_name).unwrap());
                bytes.append(&mut value.encode(huffman_encode_value).unwrap());
                bytes
            }
            HeaderRepresentation::NeverIndexedIndexedName(index, value) => {
                let mut bytes = index.encode(4).unwrap();
                bytes[0] |= 0b0001_0000;
                bytes.append(&mut value.encode(huffman_encode_value).unwrap());
                bytes
            }
            HeaderRepresentation::NeverIndexedNewName(name, value) => {
                let mut bytes: Vec<u8> = Vec::new();
                bytes.push(0b0001_0000);
                bytes.append(&mut name.encode(huffman_encode_name).unwrap());
                bytes.append(&mut value.encode(huffman_encode_value).unwrap());
                bytes
            }
            HeaderRepresentation::SizeUpdate(max_size) => {
                let mut bytes = max_size.encode(5).unwrap();
                bytes[0] |= 0b0010_0000;
                bytes
            }
        }
    }
}
