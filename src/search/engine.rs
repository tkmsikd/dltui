// Search Engine
//
// This file implements the search engine for DLT messages.

use rayon::prelude::*;
use regex::Regex;
use std::sync::Arc;

use crate::parser::{DltFile, DltMessage};

/// Search engine for DLT messages
pub struct SearchEngine {
    /// Search pattern
    pattern: Regex,
}

impl SearchEngine {
    /// Create a new search engine with the given pattern
    pub fn new(pattern: impl AsRef<str>) -> Result<Self, regex::Error> {
        let regex = Regex::new(pattern.as_ref())?;
        Ok(Self { pattern: regex })
    }

    /// Search for the pattern in a DLT file
    pub fn search(&self, file: &DltFile) -> Vec<usize> {
        // Apply the search in parallel
        (0..file.message_count())
            .into_par_iter()
            .filter_map(|idx| match file.get_message(idx) {
                Ok(msg) if self.matches(&msg) => Some(idx),
                _ => None,
            })
            .collect()
    }

    /// Search for the pattern in a list of messages
    pub fn search_in_messages(&self, messages: &[DltMessage]) -> Vec<usize> {
        // Apply the search in parallel
        (0..messages.len())
            .into_par_iter()
            .filter_map(|idx| {
                if self.matches(&messages[idx]) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Search for the pattern in a list of message indices
    pub fn search_in_indices(&self, file: &DltFile, indices: &[usize]) -> Vec<usize> {
        // Apply the search in parallel
        indices
            .par_iter()
            .filter_map(|&idx| match file.get_message(idx) {
                Ok(msg) if self.matches(&msg) => Some(idx),
                _ => None,
            })
            .collect()
    }

    /// Check if a message matches the search pattern
    pub fn matches(&self, message: &DltMessage) -> bool {
        // Check if the payload text matches the pattern
        if let Some(text) = &message.payload_text {
            return self.pattern.is_match(text);
        }

        // Check if the application ID matches the pattern
        if let Some(app_id) = message.app_id() {
            if self.pattern.is_match(&app_id) {
                return true;
            }
        }

        // Check if the context ID matches the pattern
        if let Some(ctx_id) = message.context_id() {
            if self.pattern.is_match(&ctx_id) {
                return true;
            }
        }

        // Check if the ECU ID matches the pattern
        let ecu_id = message.ecu_id();
        if self.pattern.is_match(&ecu_id) {
            return true;
        }

        false
    }

    /// Get the search pattern
    pub fn pattern(&self) -> &Regex {
        &self.pattern
    }

    /// Set the search pattern
    pub fn set_pattern(&mut self, pattern: impl AsRef<str>) -> Result<(), regex::Error> {
        self.pattern = Regex::new(pattern.as_ref())?;
        Ok(())
    }
}
