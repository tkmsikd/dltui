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
    /// Case sensitivity flag
    case_sensitive: bool,
}

impl SearchEngine {
    /// Create a new search engine with the given pattern
    pub fn new(pattern: impl AsRef<str>) -> Result<Self, regex::Error> {
        Self::with_case_sensitivity(pattern, true)
    }

    /// Create a new search engine with the given pattern and case sensitivity
    pub fn with_case_sensitivity(
        pattern: impl AsRef<str>,
        case_sensitive: bool,
    ) -> Result<Self, regex::Error> {
        let regex = if case_sensitive {
            Regex::new(pattern.as_ref())?
        } else {
            Regex::new(&format!("(?i){}", pattern.as_ref()))?
        };

        Ok(Self {
            pattern: regex,
            case_sensitive,
        })
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
        self.set_pattern_with_case_sensitivity(pattern, self.case_sensitive)
    }

    /// Set the search pattern with case sensitivity
    pub fn set_pattern_with_case_sensitivity(
        &mut self,
        pattern: impl AsRef<str>,
        case_sensitive: bool,
    ) -> Result<(), regex::Error> {
        self.case_sensitive = case_sensitive;

        self.pattern = if case_sensitive {
            Regex::new(pattern.as_ref())?
        } else {
            Regex::new(&format!("(?i){}", pattern.as_ref()))?
        };

        Ok(())
    }

    /// Get case sensitivity setting
    pub fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }

    /// Set case sensitivity
    pub fn set_case_sensitive(&mut self, case_sensitive: bool) -> Result<(), regex::Error> {
        if self.case_sensitive == case_sensitive {
            return Ok(());
        }

        // Get the current pattern as a string
        let pattern_str = self.pattern.as_str();

        // If switching from case-insensitive to case-sensitive, remove the (?i) prefix
        let pattern = if !case_sensitive && pattern_str.starts_with("(?i)") {
            pattern_str.to_string()
        } else if case_sensitive && !pattern_str.starts_with("(?i)") {
            // If switching from case-sensitive to case-insensitive, add the (?i) prefix
            format!("(?i){}", pattern_str)
        } else {
            // No change needed
            pattern_str.to_string()
        };

        self.set_pattern_with_case_sensitivity(pattern, case_sensitive)
    }
}
