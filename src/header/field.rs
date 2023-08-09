use std::fmt;

use crate::error::Http2Error;
use crate::header::primitive::HpackString;
use crate::header::representation::HeaderRepresentation;
use crate::header::table::HeaderTable;

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
    pub fn new(name: HeaderName, value: HeaderValue) -> Self {
        HeaderField { name, value }
    }

    /// Get the name of the header field.
    pub fn name(&self) -> HeaderName {
        self.name.clone()
    }

    /// Get the value of the header field.
    pub fn value(&self) -> HeaderValue {
        self.value.clone()
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

    /// Build a header field from a representation and a header table.
    ///
    /// # Arguments
    ///
    /// * `header_representation` - The representation of the header field.
    /// * `header_table` - The header table to use.
    ///
    /// # Errors
    ///
    /// * `Http2Error::InvalidHeaderRepresentation` if the representation is invalid.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(header_field))` if the representation is a header field.
    /// * `Ok(None)` if the representation is a header size update.
    pub fn from_representation(
        header_representation: HeaderRepresentation,
        header_table: &mut HeaderTable,
    ) -> Result<Option<HeaderField>, Http2Error> {
        match header_representation {
            HeaderRepresentation::Indexed(index) => {
                // Parse the index.
                let index: usize = index.try_into()?;

                // Try to retrieve the header field from the header table.
                let header_field = header_table.get(index)?;

                Ok(Some(header_field))
            }
            HeaderRepresentation::IncrementalIndexingIndexedName(index, value) => {
                // Parse the index.
                let index: usize = index.try_into()?;

                // Try to retrieve the header field name from the header table.
                let name = header_table.get(index)?.name();

                // Build the header field.
                let header_field = HeaderField::new(name, value.into());

                // Add a new entry to the header table.
                header_table.add_entry(header_field.clone());

                Ok(Some(header_field))
            }
            HeaderRepresentation::IncrementalIndexingNewName(name, value) => {
                // Build the header field.
                let header_field = HeaderField::new(name.into(), value.into());

                // Add a new entry to the header table.
                header_table.add_entry(header_field.clone());

                Ok(Some(header_field))
            }
            HeaderRepresentation::WithoutIndexingIndexedName(index, value) => {
                // Parse the index.
                let index: usize = index.try_into()?;

                // Try to retrieve the header field name from the header table.
                let name = header_table.get(index)?.name();

                // Build the header field.
                let header_field = HeaderField::new(name, value.into());

                Ok(Some(header_field))
            }
            HeaderRepresentation::WithoutIndexingNewName(name, value) => {
                // Build the header field.
                let header_field = HeaderField::new(name.into(), value.into());

                Ok(Some(header_field))
            }
            HeaderRepresentation::NeverIndexedIndexedName(index, value) => {
                // Parse the index.
                let index: usize = index.try_into()?;

                // Try to retrieve the header field name from the header table.
                let name = header_table.get(index)?.name();

                // Build the header field.
                let header_field = HeaderField::new(name, value.into());

                Ok(Some(header_field))
            }
            HeaderRepresentation::NeverIndexedNewName(name, value) => {
                // Build the header field.
                let header_field = HeaderField::new(name.into(), value.into());

                Ok(Some(header_field))
            }
            HeaderRepresentation::SizeUpdate(max_size) => {
                // Parse the maximum size.
                let max_size: usize = max_size.try_into()?;

                // Update the maximum size of the header table.
                header_table.set_max_size(max_size);

                Ok(None)
            }
        }
    }

    /// Build a representation from a header field and a header table updating
    /// the header table when possible.
    ///
    /// # Arguments
    ///
    /// * `header_table` - The header table to use.
    pub fn into_representation(&self, header_table: &mut HeaderTable) -> HeaderRepresentation {
        if let Some(index) = header_table.contains(self) {
            return HeaderRepresentation::Indexed(index.into());
        }

        if let Some(index) = header_table.contains_name(self) {
            // Add a new entry to the header table.
            header_table.add_entry(self.clone());
            return HeaderRepresentation::IncrementalIndexingIndexedName(
                index.into(),
                self.value().into(),
            );
        }

        // Add a new entry to the header table.
        header_table.add_entry(self.clone());
        HeaderRepresentation::IncrementalIndexingNewName(self.name().into(), self.value().into())
    }

    /// Build a representation from a header field and a header table without
    /// indexing the header field.
    ///
    /// # Arguments
    ///
    /// * `header_table` - The header table to use.
    pub fn into_representation_without_indexing(
        &self,
        header_table: &mut HeaderTable,
    ) -> HeaderRepresentation {
        if let Some(index) = header_table.contains_name(self) {
            return HeaderRepresentation::WithoutIndexingIndexedName(
                index.into(),
                self.value().into(),
            );
        }

        HeaderRepresentation::WithoutIndexingNewName(self.name().into(), self.value().into())
    }

    /// Build a representation from a header field and a header table never
    /// indexing the header field.
    ///
    /// # Arguments
    ///
    /// * `header_table` - The header table to use.
    pub fn into_representation_never_index(
        &self,
        header_table: &mut HeaderTable,
    ) -> HeaderRepresentation {
        if let Some(index) = header_table.contains_name(self) {
            return HeaderRepresentation::NeverIndexedIndexedName(index.into(), self.value().into());
        }

        HeaderRepresentation::NeverIndexedNewName(self.name().into(), self.value().into())
    }
}

impl From<(HeaderName, HeaderValue)> for HeaderField {
    /// Create a new HTTP/2 header field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the header field.
    /// * `value` - The value of the header field.
    fn from((name, value): (HeaderName, HeaderValue)) -> Self {
        HeaderField { name, value }
    }
}

impl fmt::Display for HeaderField {
    /// Format a header field.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

/// A HTTP/2 header field name.
#[derive(Clone, Debug, PartialEq)]
pub struct HeaderName {
    name: String,
}

impl From<&str> for HeaderName {
    /// Create a new HTTP/2 header field name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the header field.
    fn from(name: &str) -> Self {
        HeaderName {
            name: name.to_string(),
        }
    }
}

impl From<String> for HeaderName {
    /// Create a new HTTP/2 header field name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the header field.
    fn from(name: String) -> Self {
        HeaderName { name }
    }
}

impl From<HpackString> for HeaderName {
    /// Create a new HTTP/2 header field name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the header field.
    fn from(name: HpackString) -> Self {
        HeaderName { name: name.into() }
    }
}

impl From<&HpackString> for HeaderName {
    /// Create a new HTTP/2 header field name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the header field.
    fn from(name: &HpackString) -> Self {
        HeaderName { name: name.into() }
    }
}

impl Into<String> for HeaderName {
    /// Convert a header field name into a String.
    ///
    /// # Returns
    ///
    /// A String containing the header field name.
    fn into(self) -> String {
        self.name
    }
}

impl Into<HpackString> for HeaderName {
    /// Convert a header field value into a HpackString.
    ///
    /// # Returns
    ///
    /// A HpackString containing the header field value.
    fn into(self) -> HpackString {
        self.name.into()
    }
}

impl fmt::Display for HeaderName {
    /// Format a header name to be displayed.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// A HTTP/2 header field value.
#[derive(Clone, Debug, PartialEq)]
pub struct HeaderValue {
    value: String,
}

impl From<&str> for HeaderValue {
    /// Create a new HTTP/2 header field value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the header field.
    fn from(value: &str) -> Self {
        HeaderValue {
            value: value.to_string(),
        }
    }
}

impl From<String> for HeaderValue {
    /// Create a new HTTP/2 header field value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the header field.
    fn from(value: String) -> Self {
        HeaderValue { value }
    }
}

impl From<HpackString> for HeaderValue {
    /// Create a new HTTP/2 header field value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the header field.
    fn from(value: HpackString) -> Self {
        HeaderValue {
            value: value.into(),
        }
    }
}

impl From<&HpackString> for HeaderValue {
    /// Create a new HTTP/2 header field value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the header field.
    fn from(value: &HpackString) -> Self {
        HeaderValue {
            value: value.into(),
        }
    }
}

impl Into<String> for HeaderValue {
    /// Convert a header field value into a String.
    ///
    /// # Returns
    ///
    /// A String containing the header field value.
    fn into(self) -> String {
        self.value
    }
}

impl Into<HpackString> for HeaderValue {
    /// Convert a header field value into a HpackString.
    ///
    /// # Returns
    ///
    /// A HpackString containing the header field value.
    fn into(self) -> HpackString {
        self.value.into()
    }
}

impl fmt::Display for HeaderValue {
    /// Format a header name to be displayed.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
