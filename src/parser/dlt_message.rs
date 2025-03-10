// DLT Message Structures
//
// This file defines the structures for DLT messages according to the DLT specification.

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use chrono::{DateTime, TimeZone, Utc};
use std::io::{Cursor, Read, Result as IoResult};

/// DLT message log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LogLevel {
    #[default]
    Fatal,
    Error,
    Warning,
    Info,
    Debug,
    Verbose,
    Unknown(u8),
}

impl From<u8> for LogLevel {
    fn from(value: u8) -> Self {
        match value {
            1 => LogLevel::Fatal,
            2 => LogLevel::Error,
            3 => LogLevel::Warning,
            4 => LogLevel::Info,
            5 => LogLevel::Debug,
            6 => LogLevel::Verbose,
            v => LogLevel::Unknown(v),
        }
    }
}

/// DLT message types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageType {
    #[default]
    Log,
    TraceVariable,
    NetworkTrace,
    Control,
    Unknown(u8),
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => MessageType::Log,
            1 => MessageType::TraceVariable,
            2 => MessageType::NetworkTrace,
            3 => MessageType::Control,
            v => MessageType::Unknown(v),
        }
    }
}

/// DLT Storage Header (16 bytes)
#[derive(Debug, Clone)]
pub struct DltStorageHeader {
    /// "DLT" + 0x01 pattern
    pub pattern: [u8; 4],
    /// Seconds since 1970-01-01 00:00:00 UTC
    pub timestamp_seconds: u32,
    /// Microseconds (0..999999)
    pub timestamp_microseconds: u32,
    /// ECU ID (up to 4 characters)
    pub ecu_id: [u8; 4],
}

impl DltStorageHeader {
    pub fn parse(data: &mut Cursor<&[u8]>) -> IoResult<Self> {
        let mut pattern = [0u8; 4];
        data.read_exact(&mut pattern)?;

        let timestamp_seconds = data.read_u32::<BigEndian>()?;
        let timestamp_microseconds = data.read_u32::<BigEndian>()?;

        let mut ecu_id = [0u8; 4];
        data.read_exact(&mut ecu_id)?;

        Ok(Self {
            pattern,
            timestamp_seconds,
            timestamp_microseconds,
            ecu_id,
        })
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        if let Some(dt) = Utc
            .timestamp_opt(
                self.timestamp_seconds as i64,
                self.timestamp_microseconds * 1000,
            )
            .single()
        {
            dt
        } else {
            // Fallback to epoch time if timestamp is invalid
            Utc.timestamp_opt(0, 0).single().unwrap()
        }
    }

    pub fn ecu_id_str(&self) -> String {
        String::from_utf8_lossy(&self.ecu_id)
            .trim_end_matches('\0')
            .to_string()
    }

    pub fn is_valid(&self) -> bool {
        self.pattern[0] == b'D'
            && self.pattern[1] == b'L'
            && self.pattern[2] == b'T'
            && self.pattern[3] == 0x01
    }
}

/// DLT Standard Header (4 bytes)
#[derive(Debug, Clone)]
pub struct DltStandardHeader {
    /// Header type: 1 = with extended header, 0 = without
    pub use_extended_header: bool,
    /// Message counter (0..255)
    pub message_counter: u8,
    /// Overall length of the message in bytes (including all headers)
    pub length: u16,
    /// Message type (log, trace, etc.)
    pub message_type: MessageType,
    /// Version number of the DLT protocol
    pub version: u8,
}

impl DltStandardHeader {
    pub fn parse(data: &mut Cursor<&[u8]>) -> IoResult<Self> {
        let header_type = data.read_u8()?;
        let message_counter = data.read_u8()?;
        let length = data.read_u16::<LittleEndian>()?;

        // Extract fields from header_type
        let use_extended_header = (header_type & 0x01) != 0;
        let version = (header_type >> 5) & 0x07;
        let message_type_value = (header_type >> 1) & 0x07;

        Ok(Self {
            use_extended_header,
            message_counter,
            length,
            message_type: MessageType::from(message_type_value),
            version,
        })
    }
}

/// DLT Extended Header (optional, up to 10 bytes)
#[derive(Debug, Clone)]
pub struct DltExtendedHeader {
    /// Message info
    pub message_info: u8,
    /// Number of arguments
    pub argument_count: u8,
    /// Application ID (up to 4 characters)
    pub app_id: [u8; 4],
    /// Context ID (up to 4 characters)
    pub context_id: [u8; 4],
    /// Log level
    pub log_level: LogLevel,
}

impl DltExtendedHeader {
    pub fn parse(data: &mut Cursor<&[u8]>) -> IoResult<Self> {
        let message_info = data.read_u8()?;
        let argument_count = data.read_u8()?;

        let mut app_id = [0u8; 4];
        data.read_exact(&mut app_id)?;

        let mut context_id = [0u8; 4];
        data.read_exact(&mut context_id)?;

        // Extract log level from message_info
        let log_level_value = (message_info >> 4) & 0x07;

        Ok(Self {
            message_info,
            argument_count,
            app_id,
            context_id,
            log_level: LogLevel::from(log_level_value),
        })
    }

    pub fn app_id_str(&self) -> String {
        String::from_utf8_lossy(&self.app_id)
            .trim_end_matches('\0')
            .to_string()
    }

    pub fn context_id_str(&self) -> String {
        String::from_utf8_lossy(&self.context_id)
            .trim_end_matches('\0')
            .to_string()
    }
}

/// Complete DLT Message
#[derive(Debug, Clone)]
pub struct DltMessage {
    /// Storage header
    pub storage_header: DltStorageHeader,
    /// Standard header
    pub standard_header: DltStandardHeader,
    /// Extended header (optional)
    pub extended_header: Option<DltExtendedHeader>,
    /// Raw payload data
    pub payload: Vec<u8>,
    /// Parsed payload text (if available)
    pub payload_text: Option<String>,
}

impl DltMessage {
    pub fn parse(data: &[u8]) -> IoResult<Self> {
        let mut cursor = Cursor::new(data);

        let storage_header = DltStorageHeader::parse(&mut cursor)?;
        let standard_header = DltStandardHeader::parse(&mut cursor)?;

        let extended_header = if standard_header.use_extended_header {
            Some(DltExtendedHeader::parse(&mut cursor)?)
        } else {
            None
        };

        // Calculate payload size and read payload
        let headers_size = cursor.position() as usize;
        let payload_size = standard_header.length as usize - headers_size;

        let mut payload = vec![0u8; payload_size];
        cursor.read_exact(&mut payload)?;

        // Try to parse payload as text
        let payload_text = Self::parse_payload_text(&payload, &extended_header);

        Ok(Self {
            storage_header,
            standard_header,
            extended_header,
            payload,
            payload_text,
        })
    }

    fn parse_payload_text(
        payload: &[u8],
        _extended_header: &Option<DltExtendedHeader>,
    ) -> Option<String> {
        // Simple heuristic: if it looks like ASCII/UTF-8 text, return it as a string
        if payload
            .iter()
            .all(|&b| b >= 32 && b < 127 || b == b'\n' || b == b'\r' || b == b'\t')
        {
            String::from_utf8(payload.to_vec()).ok()
        } else {
            None
        }
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.storage_header.timestamp()
    }

    pub fn ecu_id(&self) -> String {
        self.storage_header.ecu_id_str()
    }

    pub fn app_id(&self) -> Option<String> {
        self.extended_header.as_ref().map(|h| h.app_id_str())
    }

    pub fn context_id(&self) -> Option<String> {
        self.extended_header.as_ref().map(|h| h.context_id_str())
    }

    pub fn log_level(&self) -> Option<LogLevel> {
        self.extended_header.as_ref().map(|h| h.log_level)
    }

    pub fn message_type(&self) -> MessageType {
        self.standard_header.message_type
    }

    pub fn payload_as_text(&self) -> String {
        self.payload_text.clone().unwrap_or_else(|| {
            // Fallback to hex representation
            payload_to_hex_string(&self.payload)
        })
    }
}

fn payload_to_hex_string(payload: &[u8]) -> String {
    let mut result = String::new();
    for (i, chunk) in payload.chunks(16).enumerate() {
        if i > 0 {
            result.push('\n');
        }

        // Offset
        result.push_str(&format!("{:08x}  ", i * 16));

        // Hex bytes
        for (j, &byte) in chunk.iter().enumerate() {
            if j == 8 {
                result.push(' ');
            }
            result.push_str(&format!("{:02x} ", byte));
        }

        // Padding for incomplete lines
        if chunk.len() < 16 {
            for _ in 0..(16 - chunk.len()) {
                result.push_str("   ");
            }
            if chunk.len() < 8 {
                result.push(' ');
            }
        }

        // ASCII representation
        result.push_str(" |");
        for &byte in chunk {
            if byte >= 32 && byte < 127 {
                result.push(byte as char);
            } else {
                result.push('.');
            }
        }
        result.push('|');
    }

    result
}
