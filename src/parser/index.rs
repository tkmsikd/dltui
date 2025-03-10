// DLT File Indexing
//
// This file provides functionality for indexing DLT files to enable
// fast message lookup and filtering.

use std::collections::HashMap;
use std::sync::Arc;

use crate::parser::{DltFile, LogLevel, Result};

/// Index for DLT messages
pub struct Index {
    /// Reference to the DLT file
    file: Arc<DltFile>,
    /// Map of application IDs to message indices
    app_id_index: HashMap<String, Vec<usize>>,
    /// Map of context IDs to message indices
    context_id_index: HashMap<String, Vec<usize>>,
    /// Map of log levels to message indices
    log_level_index: HashMap<LogLevel, Vec<usize>>,
    /// Map of ECU IDs to message indices
    ecu_id_index: HashMap<String, Vec<usize>>,
}

impl Index {
    /// Create a new index for a DLT file
    pub fn new(file: Arc<DltFile>) -> Result<Self> {
        let mut index = Self {
            file: file.clone(),
            app_id_index: HashMap::new(),
            context_id_index: HashMap::new(),
            log_level_index: HashMap::new(),
            ecu_id_index: HashMap::new(),
        };

        // Build the indices
        index.build()?;

        Ok(index)
    }

    /// Build all indices
    fn build(&mut self) -> Result<()> {
        let message_count = self.file.message_count();

        for idx in 0..message_count {
            let message = self.file.get_message(idx)?;

            // Index by ECU ID
            let ecu_id = message.ecu_id();
            self.ecu_id_index.entry(ecu_id).or_default().push(idx);

            // Index by application ID (if available)
            if let Some(app_id) = message.app_id() {
                self.app_id_index.entry(app_id).or_default().push(idx);
            }

            // Index by context ID (if available)
            if let Some(context_id) = message.context_id() {
                self.context_id_index
                    .entry(context_id)
                    .or_default()
                    .push(idx);
            }

            // Index by log level (if available)
            if let Some(log_level) = message.log_level() {
                self.log_level_index.entry(log_level).or_default().push(idx);
            }
        }

        Ok(())
    }

    /// Get all unique application IDs
    pub fn app_ids(&self) -> Vec<String> {
        self.app_id_index.keys().cloned().collect()
    }

    /// Get all unique context IDs
    pub fn context_ids(&self) -> Vec<String> {
        self.context_id_index.keys().cloned().collect()
    }

    /// Get all unique ECU IDs
    pub fn ecu_ids(&self) -> Vec<String> {
        self.ecu_id_index.keys().cloned().collect()
    }

    /// Get all messages with a specific application ID
    pub fn messages_by_app_id(&self, app_id: &str) -> Vec<usize> {
        self.app_id_index.get(app_id).cloned().unwrap_or_default()
    }

    /// Get all messages with a specific context ID
    pub fn messages_by_context_id(&self, context_id: &str) -> Vec<usize> {
        self.context_id_index
            .get(context_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all messages with a specific log level
    pub fn messages_by_log_level(&self, log_level: LogLevel) -> Vec<usize> {
        self.log_level_index
            .get(&log_level)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all messages with a specific ECU ID
    pub fn messages_by_ecu_id(&self, ecu_id: &str) -> Vec<usize> {
        self.ecu_id_index.get(ecu_id).cloned().unwrap_or_default()
    }

    /// Get the DLT file
    pub fn file(&self) -> &DltFile {
        &self.file
    }
}
