use std::fmt;

use crate::{error::Http2Error, frame::FrameHeader};

/// SETTINGS Frame flags.
#[derive(Debug, PartialEq)]
pub enum SettingsFlag {
    Ack,
}

impl SettingsFlag {
    /// Parse the flags from a byte.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to parse the flags from.
    pub fn parse_flags(byte: u8) -> Vec<SettingsFlag> {
        let mut flags: Vec<SettingsFlag> = Vec::new();

        if byte & 0x1 != 0 {
            flags.push(SettingsFlag::Ack);
        }

        flags
    }
}

/// SETTINGS Frame parameters.
#[derive(Debug)]
pub enum SettingsParameter {
    HeaderTableSize(u32),
    EnablePush(u32),
    MaxConcurrentStreams(u32),
    InitialWindowSize(u32),
    MaxFrameSize(u32),
    MaxHeaderListSize(u32),
}

impl SettingsParameter {
    /// Deserialize a vector of bytes to a vector of SETTINGS parameters.
    /// 
    /// # Arguments
    /// 
    /// * `bytes` - The vector of bytes to deserialize.
    pub fn deserialize(bytes: Vec<u8>) -> Result<Vec<SettingsParameter>, Http2Error> {
        // Check that the length is valid.
        if bytes.len() % 6 != 0 {
            return Err(Http2Error::FrameError(format!(
                "Invalid length for SETTINGS parameter: {}",
                bytes.len()
            )));
        }

        // Deserialize the parameters.
        let mut settings_parameters: Vec<SettingsParameter> = Vec::new();

        let mut bytes = bytes;
        while bytes.len() != 0 {
            match u16::from_be_bytes([bytes[0], bytes[1]]) {
                0x1 => {
                    settings_parameters.push(SettingsParameter::HeaderTableSize(
                        u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]),
                    ));
                }
                0x2 => {
                    settings_parameters.push(SettingsParameter::EnablePush(
                        u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]),
                    ));
                }
                0x3 => {
                    settings_parameters.push(SettingsParameter::MaxConcurrentStreams(
                        u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]),
                    ));
                }
                0x4 => {
                    settings_parameters.push(SettingsParameter::InitialWindowSize(
                        u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]),
                    ));
                }
                0x5 => {
                    settings_parameters.push(SettingsParameter::MaxFrameSize(
                        u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]),
                    ));
                }
                0x6 => {
                    settings_parameters.push(SettingsParameter::MaxHeaderListSize(
                        u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]),
                    ));
                }
                _ => {
                    return Err(Http2Error::FrameError(format!(
                        "Invalid SETTINGS parameter: {}",
                        u16::from_be_bytes([bytes[0], bytes[1]])
                    )));
                }
            }

            bytes = bytes[6..].to_vec();
        }

        Ok(settings_parameters)
    }
}

/// SETTINGS Frame payload.
///
/// The payload of a SETTINGS frame consists of zero or more parameters,
/// each consisting of an unsigned 16-bit setting identifier and an
/// unsigned 32-bit value.
///
///  +-------------------------------+
///  |       Identifier (16)         |
///  +-------------------------------+-------------------------------+
///  |                        Value (32)                             |
///  +---------------------------------------------------------------+
#[derive(Debug)]
pub struct Settings {
    flags: Vec<SettingsFlag>,
    ack: bool,
    settings_parameters: Vec<SettingsParameter>,
}

impl Settings {
    /// Deserialize a DATA frame from a frame header and a payload.
    ///
    /// # Arguments
    ///
    /// * `header` - The frame header.
    /// * `payload` - The frame payload.
    pub fn deserialize(header: &FrameHeader, payload: Vec<u8>) -> Result<Self, Http2Error> {
        // Deserialize the flags from the header.
        let flags: Vec<SettingsFlag> = SettingsFlag::parse_flags(header.flags());

        let mut ack: bool = false;
        if flags.contains(&SettingsFlag::Ack) {
            ack = true;
            if payload.len() != 0 {
                return Err(Http2Error::FrameError(
                    "Invalid payload length: SETTINGS frame with ACK flag must be empty".to_string(),
                ));
            }
        }

        // Deserialize the settings parameters.
        let settings_parameters = SettingsParameter::deserialize(payload)?;

        Ok(Self {
            flags,
            ack,
            settings_parameters,
        })
    }
}

impl fmt::Display for Settings {
    /// Format a DATA frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SETTINGS Frame\n")?;
        write!(f, "Flags: {:?}\n", self.flags)?;
        write!(f, "Ack: {}\n", self.ack)?;
        write!(f, "Parameters:\n")?;
        for parameter in &self.settings_parameters {
            match parameter {
                SettingsParameter::HeaderTableSize(value) => {
                    write!(f, "Header Table Size: {}\n", value)?;
                }
                SettingsParameter::EnablePush(value) => {
                    write!(f, "Enable Push: {}\n", value)?;
                }
                SettingsParameter::MaxConcurrentStreams(value) => {
                    write!(f, "Max Concurrent Streams: {}\n", value)?;
                }
                SettingsParameter::InitialWindowSize(value) => {
                    write!(f, "Initial Window Size: {}\n", value)?;
                }
                SettingsParameter::MaxFrameSize(value) => {
                    write!(f, "Max Frame Size: {}\n", value)?;
                }
                SettingsParameter::MaxHeaderListSize(value) => {
                    write!(f, "Max Header List Size: {}\n", value)?;
                }
            }
        }
        Ok(())
    }
}
