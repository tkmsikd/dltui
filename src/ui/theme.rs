// UI Theme
//
// This file defines the color theme for the application.

use crate::parser::LogLevel;
use ratatui::style::{Color, Style};

/// UI Theme
pub struct Theme {
    /// Background color
    pub background: Color,
    /// Foreground color
    pub foreground: Color,
    /// Highlight color
    pub highlight: Color,
    /// Selected item background color
    pub selected_bg: Color,
    /// Selected item foreground color
    pub selected_fg: Color,
    /// Status bar background color
    pub status_bar_bg: Color,
    /// Status bar foreground color
    pub status_bar_fg: Color,
    /// Command line background color
    pub command_line_bg: Color,
    /// Command line foreground color
    pub command_line_fg: Color,
    /// Error color
    pub error: Color,
    /// Warning color
    pub warning: Color,
    /// Info color
    pub info: Color,
    /// Debug color
    pub debug: Color,
    /// Verbose color
    pub verbose: Color,
    /// Fatal color
    pub fatal: Color,
    /// Border color
    pub border: Color,
    /// Title color
    pub title: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            highlight: Color::Yellow,
            selected_bg: Color::DarkGray,
            selected_fg: Color::White,
            status_bar_bg: Color::Blue,
            status_bar_fg: Color::White,
            command_line_bg: Color::DarkGray,
            command_line_fg: Color::White,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Green,
            debug: Color::Cyan,
            verbose: Color::Gray,
            fatal: Color::Magenta,
            border: Color::Gray,
            title: Color::Blue,
        }
    }
}

impl Theme {
    /// Get the style for a log level
    pub fn style_for_log_level(&self, level: Option<LogLevel>) -> Style {
        match level {
            Some(LogLevel::Fatal) => Style::default().fg(self.fatal),
            Some(LogLevel::Error) => Style::default().fg(self.error),
            Some(LogLevel::Warning) => Style::default().fg(self.warning),
            Some(LogLevel::Info) => Style::default().fg(self.info),
            Some(LogLevel::Debug) => Style::default().fg(self.debug),
            Some(LogLevel::Verbose) => Style::default().fg(self.verbose),
            _ => Style::default().fg(self.foreground),
        }
    }

    /// Get the style for selected items
    pub fn selected_style(&self) -> Style {
        Style::default().bg(self.selected_bg).fg(self.selected_fg)
    }

    /// Get the style for the status bar
    pub fn status_bar_style(&self) -> Style {
        Style::default()
            .bg(self.status_bar_bg)
            .fg(self.status_bar_fg)
    }

    /// Get the style for the command line
    pub fn command_line_style(&self) -> Style {
        Style::default()
            .bg(self.command_line_bg)
            .fg(self.command_line_fg)
    }

    /// Get the style for borders
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Get the style for titles
    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.title)
            .add_modifier(ratatui::style::Modifier::BOLD)
    }

    /// Get the style for highlighted text
    pub fn highlight_style(&self) -> Style {
        Style::default().fg(self.highlight)
    }
}
