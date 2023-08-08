use std::fmt;

use crate::error::Http2Error;
use crate::header::field::HeaderField;
use crate::header::representation::HeaderRepresentation;
use crate::header::table::HeaderTable;


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

    /// Decode a header list from a byte vector and a header table.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte vector to decode from.
    /// * `header_table` - The header table to use.
    pub fn decode(bytes: &mut Vec<u8>, header_table: &mut HeaderTable) -> Result<Self, Http2Error> {
        let mut headers: Vec<HeaderField> = Vec::new();

        // While the provided byte vector is not empty.
        while !bytes.is_empty() {
            // Decode the header representation.
            let header_representation = HeaderRepresentation::decode(bytes)?;

            // Try to build a header field from the header representation.
            // Do nothing if the header representation was not a header field.
            match HeaderField::from_representation(header_representation, header_table)? {
                Some(header_field) => headers.push(header_field),
                None => (),
            }
        }

        Ok(Self { header_fields: headers })
    }

    /// Encode a header list into a byte vector.
    /// 
    /// # Arguments
    /// 
    /// * `header_table` - The header table to use.
    /// 
    /// # Returns
    /// 
    /// A byte vector containing the encoded header list.
    pub fn encode(&self, header_table: &mut HeaderTable) -> Result<Vec<u8>, Http2Error> {
        let mut bytes: Vec<u8> = Vec::new();

        // For each header field in the header list.
        for header_field in &self.header_fields {
            // Builds a header representation from the header field.
            let header_representation = header_field.into_representation(header_table);

            // Encode the header representation. TODO: Manage Huffman encoding.
            bytes.append(&mut header_representation.encode(false));
        }

        Ok(bytes)
    }
}

impl From<Vec<HeaderField>> for HeaderList {
    /// Create a header list from a vector of header fields.
    /// 
    /// # Arguments
    /// 
    /// * `header_fields` - The vector of header fields.
    fn from(header_fields: Vec<HeaderField>) -> Self {
        Self::new(header_fields)
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
