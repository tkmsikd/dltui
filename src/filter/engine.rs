// Filter Engine
//
// This file implements the filter engine for DLT messages.

use rayon::prelude::*;
use std::sync::Arc;

use crate::filter::FilterCriteria;
use crate::parser::{DltFile, DltMessage};

/// Filter engine for DLT messages
pub struct FilterEngine {
    /// Filter criteria
    criteria: FilterCriteria,
}

impl FilterEngine {
    /// Create a new filter engine with the given criteria
    pub fn new(criteria: FilterCriteria) -> Self {
        Self { criteria }
    }

    /// Apply the filter to a DLT file
    pub fn apply(&self, file: &DltFile) -> Vec<usize> {
        // If no filter is set, return all messages
        if self.criteria.is_empty() {
            return (0..file.message_count()).collect();
        }

        // Apply the filter in parallel
        (0..file.message_count())
            .into_par_iter()
            .filter_map(|idx| match file.get_message(idx) {
                Ok(msg) if self.matches(&msg) => Some(idx),
                _ => None,
            })
            .collect()
    }

    /// Apply the filter to a list of messages
    pub fn apply_to_messages(&self, messages: &[DltMessage]) -> Vec<usize> {
        // If no filter is set, return all messages
        if self.criteria.is_empty() {
            return (0..messages.len()).collect();
        }

        // Apply the filter in parallel
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

    /// Check if a message matches the filter criteria
    pub fn matches(&self, message: &DltMessage) -> bool {
        self.criteria.matches(message)
    }

    /// Get the filter criteria
    pub fn criteria(&self) -> &FilterCriteria {
        &self.criteria
    }

    /// Set the filter criteria
    pub fn set_criteria(&mut self, criteria: FilterCriteria) {
        self.criteria = criteria;
    }

    /// Clear the filter criteria
    pub fn clear(&mut self) {
        self.criteria.clear();
    }
}
