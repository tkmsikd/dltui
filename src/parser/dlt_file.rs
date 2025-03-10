// DLT File Handler
//
// This file provides functionality for reading and parsing DLT files.
// It uses memory mapping for efficient file access and builds an index
// for fast message lookup.

use crate::parser::{DltMessage, Error, Result};
use byteorder::ReadBytesExt;
use memmap2::{Mmap, MmapOptions};
use rayon::prelude::*;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// DLT file handler
pub struct DltFile {
    /// Path to the DLT file
    path: PathBuf,
    /// Memory-mapped file data
    mmap: Arc<Mmap>,
    /// Index of message positions in the file
    index: Vec<u64>,
    /// Total number of messages
    message_count: usize,
}

impl DltFile {
    /// Open a DLT file and build its index
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path)?;

        // Memory map the file
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let mmap = Arc::new(mmap);

        // Build the index
        let index = Self::build_index(&mmap)?;
        let message_count = index.len();

        Ok(Self {
            path,
            mmap,
            index,
            message_count,
        })
    }

    /// Build an index of message positions in the file
    fn build_index(mmap: &Mmap) -> Result<Vec<u64>> {
        let mut index = Vec::new();
        let mut pos = 0;

        while pos < mmap.len() {
            // Check if we have enough bytes for a storage header (16 bytes)
            if pos + 16 > mmap.len() {
                break;
            }

            // Check for DLT pattern
            if mmap[pos] == b'D'
                && mmap[pos + 1] == b'L'
                && mmap[pos + 2] == b'T'
                && mmap[pos + 3] == 0x01
            {
                index.push(pos as u64);

                // Read the standard header to get the message length
                if pos + 20 <= mmap.len() {
                    let mut cursor = Cursor::new(&mmap[pos + 16..pos + 20]);
                    let _header_type = match cursor.read_u8() {
                        Ok(v) => v,
                        Err(_) => 0,
                    };
                    let _message_counter = match cursor.read_u8() {
                        Ok(v) => v,
                        Err(_) => 0,
                    };
                    let mut length_bytes = [0u8; 2];
                    let _ = cursor.read_exact(&mut length_bytes);
                    let length = u16::from_le_bytes(length_bytes) as usize;

                    // Skip to the next message
                    if length > 0 && pos + length <= mmap.len() {
                        pos += length;
                        continue;
                    }
                }
            }

            // If we couldn't parse the message length or the pattern didn't match,
            // move forward one byte and try again
            pos += 1;
        }

        Ok(index)
    }

    /// Get the total number of messages in the file
    pub fn message_count(&self) -> usize {
        self.message_count
    }

    /// Get the file path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get a message by its index
    pub fn get_message(&self, idx: usize) -> Result<DltMessage> {
        if idx >= self.message_count {
            return Err(Error::NotFound(format!(
                "Message index out of bounds: {}",
                idx
            )));
        }

        let pos = self.index[idx] as usize;

        // Find the end of this message (start of the next message or end of file)
        let next_pos = if idx + 1 < self.message_count {
            self.index[idx + 1] as usize
        } else {
            self.mmap.len()
        };

        // Parse the message
        let data = &self.mmap[pos..next_pos];
        let message = DltMessage::parse(data).map_err(|e| {
            Error::Format(format!("Failed to parse message at index {}: {}", idx, e))
        })?;

        Ok(message)
    }

    /// Get multiple messages in a range
    pub fn get_messages(&self, start: usize, count: usize) -> Result<Vec<DltMessage>> {
        let end = std::cmp::min(start + count, self.message_count);

        (start..end)
            .into_par_iter()
            .map(|idx| self.get_message(idx))
            .collect()
    }

    /// Filter messages based on a predicate function
    pub fn filter<F>(&self, predicate: F) -> Vec<usize>
    where
        F: Fn(&DltMessage) -> bool + Send + Sync,
    {
        (0..self.message_count)
            .into_par_iter()
            .filter_map(|idx| match self.get_message(idx) {
                Ok(msg) if predicate(&msg) => Some(idx),
                _ => None,
            })
            .collect()
    }
}
