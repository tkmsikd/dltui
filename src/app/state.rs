// Application State
//
// This file defines the main application state and logic.

use std::path::PathBuf;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use regex::Regex;

use crate::parser::{DltFile, DltMessage, Index, LogLevel, MessageType, Result as ParserResult};
use crate::search::SearchEngine;

/// View mode for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// List view showing multiple messages
    List,
    /// Detail view showing a single message
    Detail,
    /// Help view showing keyboard shortcuts
    Help,
}

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
}

/// Input mode for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Normal mode (keyboard shortcuts)
    Normal,
    /// Search mode (typing a search pattern)
    Search,
    /// Filter mode (typing a filter pattern)
    Filter,
}

/// Application state
pub struct App {
    /// List of loaded DLT files
    pub files: Vec<Arc<DltFile>>,
    /// Indices for each file
    pub indices: Vec<Arc<Index>>,
    /// Currently selected file index
    pub current_file_idx: usize,
    /// Filter criteria
    pub filter: FilterCriteria,
    /// Filtered message indices
    pub filtered_messages: Vec<usize>,
    /// Currently selected message index
    pub selected_message_idx: usize,
    /// Current view mode
    pub view_mode: ViewMode,
    /// Current input mode
    pub input_mode: InputMode,
    /// Search engine
    pub search_engine: Option<SearchEngine>,
    /// Search pattern
    pub search_pattern: Option<Regex>,
    /// Search results (indices into filtered_messages)
    pub search_results: Vec<usize>,
    /// Current search result index
    pub current_search_idx: usize,
    /// Command input buffer
    pub command_input: String,
    /// Status message
    pub status_message: String,
    /// Should the application exit
    pub should_exit: bool,
}

impl App {
    /// Create a new application instance
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            indices: Vec::new(),
            current_file_idx: 0,
            filter: FilterCriteria::default(),
            filtered_messages: Vec::new(),
            selected_message_idx: 0,
            view_mode: ViewMode::List,
            input_mode: InputMode::Normal,
            search_engine: None,
            search_pattern: None,
            search_results: Vec::new(),
            current_search_idx: 0,
            command_input: String::new(),
            status_message: String::new(),
            should_exit: false,
        }
    }

    /// Load a DLT file
    pub fn load_file(&mut self, path: PathBuf) -> ParserResult<()> {
        // Load the file
        let file = Arc::new(DltFile::open(path)?);
        let index = Arc::new(Index::new(file.clone())?);

        // Add to the list of files
        self.files.push(file);
        self.indices.push(index);

        // Set as the current file if it's the first one
        if self.files.len() == 1 {
            self.current_file_idx = 0;
            self.apply_filter();
        }

        Ok(())
    }

    /// Apply the current filter to the current file
    pub fn apply_filter(&mut self) {
        if self.files.is_empty() {
            self.filtered_messages = Vec::new();
            return;
        }

        let file = &self.files[self.current_file_idx];

        // Apply the filter
        self.filtered_messages = file.filter(|msg| self.filter.matches(msg));

        // Reset selection
        self.selected_message_idx = 0;
        self.search_results = Vec::new();
        self.current_search_idx = 0;
    }

    /// Get the currently selected message
    pub fn selected_message(&self) -> Option<DltMessage> {
        if self.files.is_empty() || self.filtered_messages.is_empty() {
            return None;
        }

        let file = &self.files[self.current_file_idx];
        let idx = self.filtered_messages[self.selected_message_idx];
        file.get_message(idx).ok()
    }

    /// Search for a pattern in the filtered messages
    pub fn search(&mut self, pattern: &str) -> Result<(), regex::Error> {
        // Create or update the search engine
        if let Some(engine) = &mut self.search_engine {
            engine.set_pattern(pattern)?;
        } else {
            self.search_engine = Some(SearchEngine::new(pattern)?);
        }

        // Store the search pattern
        let regex = Regex::new(pattern)?;
        self.search_pattern = Some(regex);

        // Find matches
        self.search_results = Vec::new();

        if self.files.is_empty() || self.filtered_messages.is_empty() {
            return Ok(());
        }

        let file = &self.files[self.current_file_idx];
        let engine = self.search_engine.as_ref().unwrap();

        // Use the search engine to find matches
        for (i, &idx) in self.filtered_messages.iter().enumerate() {
            if let Ok(msg) = file.get_message(idx) {
                if engine.matches(&msg) {
                    self.search_results.push(i);
                }
            }
        }

        // Update status message
        if self.search_results.is_empty() {
            self.status_message = format!("No matches found for '{}'", pattern);
        } else {
            self.status_message = format!(
                "Found {} matches for '{}'",
                self.search_results.len(),
                pattern
            );
        }

        // Reset search index
        self.current_search_idx = 0;

        // Select the first result if any
        if !self.search_results.is_empty() {
            self.selected_message_idx = self.search_results[0];
        }

        Ok(())
    }

    /// Move to the next search result
    pub fn next_search_result(&mut self) {
        if self.search_results.is_empty() {
            return;
        }

        self.current_search_idx = (self.current_search_idx + 1) % self.search_results.len();
        self.selected_message_idx = self.search_results[self.current_search_idx];
    }

    /// Move to the previous search result
    pub fn prev_search_result(&mut self) {
        if self.search_results.is_empty() {
            return;
        }

        self.current_search_idx = if self.current_search_idx == 0 {
            self.search_results.len() - 1
        } else {
            self.current_search_idx - 1
        };

        self.selected_message_idx = self.search_results[self.current_search_idx];
    }

    /// Move the selection up
    pub fn move_up(&mut self) {
        if self.selected_message_idx > 0 {
            self.selected_message_idx -= 1;
        }
    }

    /// Move the selection down
    pub fn move_down(&mut self) {
        if !self.filtered_messages.is_empty()
            && self.selected_message_idx < self.filtered_messages.len() - 1
        {
            self.selected_message_idx += 1;
        }
    }

    /// Move the selection to the top
    pub fn move_to_top(&mut self) {
        self.selected_message_idx = 0;
    }

    /// Move the selection to the bottom
    pub fn move_to_bottom(&mut self) {
        if !self.filtered_messages.is_empty() {
            self.selected_message_idx = self.filtered_messages.len() - 1;
        }
    }

    /// Switch to the next file
    pub fn next_file(&mut self) {
        if self.files.len() > 1 {
            self.current_file_idx = (self.current_file_idx + 1) % self.files.len();
            self.apply_filter();
        }
    }

    /// Switch to the previous file
    pub fn prev_file(&mut self) {
        if self.files.len() > 1 {
            self.current_file_idx = if self.current_file_idx == 0 {
                self.files.len() - 1
            } else {
                self.current_file_idx - 1
            };
            self.apply_filter();
        }
    }

    /// Toggle the view mode between list and detail
    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::List => ViewMode::Detail,
            ViewMode::Detail => ViewMode::List,
            ViewMode::Help => ViewMode::List,
        };
    }

    /// Show the help view
    pub fn show_help(&mut self) {
        self.view_mode = ViewMode::Help;
    }

    /// Enter search mode
    pub fn enter_search_mode(&mut self) {
        self.input_mode = InputMode::Search;
        self.command_input = String::new();
        self.status_message = "Search: ".to_string();
    }

    /// Exit search mode
    pub fn exit_search_mode(&mut self) {
        self.input_mode = InputMode::Normal;
        self.command_input = String::new();
        self.status_message = String::new();
    }

    /// Handle search input
    pub fn handle_search_input(&mut self, key: char) {
        match key {
            '\n' | '\r' => {
                // Execute search on Enter
                let pattern = self.command_input.clone();
                if !pattern.is_empty() {
                    if let Err(e) = self.search(&pattern) {
                        self.status_message = format!("Invalid search pattern: {}", e);
                    }
                }
                self.exit_search_mode();
            }
            '\u{8}' | '\u{7f}' => {
                // Backspace
                self.command_input.pop();
            }
            '\u{1b}' => {
                // Escape
                self.exit_search_mode();
            }
            _ => {
                // Add character to input
                self.command_input.push(key);
            }
        }
    }

    /// Exit the application
    pub fn exit(&mut self) {
        self.should_exit = true;
    }
}
