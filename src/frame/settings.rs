use std::fmt;

use crate::error::Http2Error;
use crate::frame::{FrameFlag, FrameHeader};

#[derive(Debug, PartialEq)]
pub enum SettingsParameter {
    HeaderTableSize(u32),
    EnablePush(u32),
    MaxConcurrentStreams(u32),
    InitialWindowSize(u32),
    MaxFrameSize(u32),
    MaxHeaderListSize(u32),
}

impl SettingsParameter {
    pub fn deserialize(parameter_id: u16, parameter_value: u32) -> Result<Self, Http2Error> {
        match parameter_id {
            0x1 => Ok(Self::HeaderTableSize(parameter_value)),
            0x2 => Ok(Self::EnablePush(parameter_value)),
            0x3 => Ok(Self::MaxConcurrentStreams(parameter_value)),
            0x4 => Ok(Self::InitialWindowSize(parameter_value)),
            0x5 => Ok(Self::MaxFrameSize(parameter_value)),
            0x6 => Ok(Self::MaxHeaderListSize(parameter_value)),
            _ => Err(Http2Error::FrameError(format!(
                "Invalid SETTINGS parameter: {}",
                parameter_id
            ))),
        }
    }
}

impl fmt::Display for SettingsParameter {
    /// Format a SETTINGS parameter.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingsParameter::HeaderTableSize(value) => write!(f, "Header Table Size: {}", value),
            SettingsParameter::EnablePush(value) => write!(f, "Enable Push: {}", value),
            SettingsParameter::MaxConcurrentStreams(value) => {
                write!(f, "Max Concurrent Streams: {}", value)
            }
            SettingsParameter::InitialWindowSize(value) => {
                write!(f, "Initial Window Size: {}", value)
            }
            SettingsParameter::MaxFrameSize(value) => write!(f, "Max Frame Size: {}", value),
            SettingsParameter::MaxHeaderListSize(value) => {
                write!(f, "Max Header List Size: {}", value)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SettingsFrame {
    ack: bool,
    settings_parameters: Vec<SettingsParameter>,
}

impl SettingsFrame {
    pub fn deserialize_flags(byte: u8) -> Vec<FrameFlag> {
        let mut frame_flags = Vec::new();

        if (byte & 0x01) != 0 {
            frame_flags.push(FrameFlag::Ack);
        }

        frame_flags
    }

    pub fn deserialize(
        frame_header: &FrameHeader,
        bytes: &mut Vec<u8>,
    ) -> Result<Self, Http2Error> {
        // Check that the payload length is valid.
        if frame_header.payload_length() % 6 != 0 {
            return Err(Http2Error::FrameError(format!(
                "Invalid length for SETTINGS parameter: {}",
                bytes.len()
            )));
        }

        // Deserialize the flags from the header.
        let flags: Vec<FrameFlag> = SettingsFrame::deserialize_flags(frame_header.frame_flags());

        // Deserialize the parameters.
        let mut settings_parameters: Vec<SettingsParameter> = Vec::new();

        while settings_parameters.len() != (frame_header.payload_length() / 6) as usize {
            let parameter_id = u16::from_be_bytes([bytes[0], bytes[1]]);
            let parameter_value = u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);

            // Remove the parameter from the bytes stream.
            *bytes = bytes[6..].to_vec();

            // Check that the parameter is valid.
            settings_parameters.push(SettingsParameter::deserialize(
                parameter_id,
                parameter_value,
            )?);
        }

        Ok(Self {
            ack: flags.contains(&FrameFlag::Ack),
            settings_parameters,
        })
    }
}

impl fmt::Display for SettingsFrame {
    /// Format a DATA frame.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SETTINGS\n")?;
        write!(f, "Ack: {}\n", self.ack)?;
        write!(f, "Parameters: {:?}", self.settings_parameters)
    }
}
