// Filter Criteria
//
// This file defines the filter criteria for DLT messages.

use chrono::{DateTime, Utc};
use regex::Regex;

use crate::parser::{DltMessage, LogLevel, MessageType};

/// Filter criteria for DLT messages
#[derive(Debug, Clone)]
pub struct FilterCriteria {
    /// Filter by application ID
    pub app_id: Option<String>,
    /// Filter by context ID
    pub context_id: Option<String>,
    /// Filter by log level
    pub log_level: Option<LogLevel>,
    /// Filter by time range
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filter by message type
    pub message_type: Option<MessageType>,
    /// Filter by text pattern
    pub text_pattern: Option<Regex>,
}

impl Default for FilterCriteria {
    fn default() -> Self {
        Self {
            app_id: None,
            context_id: None,
            log_level: None,
            time_range: None,
            message_type: None,
            text_pattern: None,
        }
    }
}

impl FilterCriteria {
    /// Create a new filter criteria
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a message matches the filter criteria
    pub fn matches(&self, message: &DltMessage) -> bool {
        // Check application ID
        if let Some(app_id) = &self.app_id {
            if message.app_id().as_ref().map_or(true, |id| id != app_id) {
                return false;
            }
        }

        // Check context ID
        if let Some(context_id) = &self.context_id {
            if message
                .context_id()
                .as_ref()
                .map_or(true, |id| id != context_id)
            {
                return false;
            }
        }

        // Check log level
        if let Some(log_level) = &self.log_level {
            if message
                .log_level()
                .map_or(true, |level| &level != log_level)
            {
                return false;
            }
        }

        // Check time range
        if let Some((start, end)) = &self.time_range {
            let timestamp = message.timestamp();
            if timestamp < *start || timestamp > *end {
                return false;
            }
        }

        // Check message type
        if let Some(message_type) = &self.message_type {
            if &message.message_type() != message_type {
                return false;
            }
        }

        // Check text pattern
        if let Some(pattern) = &self.text_pattern {
            if let Some(text) = &message.payload_text {
                if !pattern.is_match(text) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Set the application ID filter
    pub fn with_app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    /// Set the context ID filter
    pub fn with_context_id(mut self, context_id: impl Into<String>) -> Self {
        self.context_id = Some(context_id.into());
        self
    }

    /// Set the log level filter
    pub fn with_log_level(mut self, log_level: LogLevel) -> Self {
        self.log_level = Some(log_level);
        self
    }

    /// Set the time range filter
    pub fn with_time_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.time_range = Some((start, end));
        self
    }

    /// Set the message type filter
    pub fn with_message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = Some(message_type);
        self
    }

    /// Set the text pattern filter
    pub fn with_text_pattern(mut self, pattern: impl AsRef<str>) -> Result<Self, regex::Error> {
        let regex = Regex::new(pattern.as_ref())?;
        self.text_pattern = Some(regex);
        Ok(self)
    }

    /// Clear all filters
    pub fn clear(&mut self) {
        self.app_id = None;
        self.context_id = None;
        self.log_level = None;
        self.time_range = None;
        self.message_type = None;
        self.text_pattern = None;
    }

    /// Check if any filter is set
    pub fn is_empty(&self) -> bool {
        self.app_id.is_none()
            && self.context_id.is_none()
            && self.log_level.is_none()
            && self.time_range.is_none()
            && self.message_type.is_none()
            && self.text_pattern.is_none()
    }
}
