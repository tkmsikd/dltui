// DLT Parser Module
//
// This module is responsible for parsing DLT (Diagnostic Log and Trace) files.
// It provides functionality to read, parse, and access DLT messages.

mod dlt_file;
mod dlt_message;
mod index;

pub use dlt_file::DltFile;
pub use dlt_message::{DltMessage, LogLevel, MessageType};
pub use index::Index;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid DLT format: {0}")]
    Format(String),

    #[error("Index error: {0}")]
    Index(String),

    #[error("Message not found: {0}")]
    NotFound(String),
}
