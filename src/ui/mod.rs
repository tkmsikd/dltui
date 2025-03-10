// UI Module
//
// This module handles the user interface using the ratatui library.

mod event;
mod theme;
mod views;

pub use event::{Event, EventHandler};
pub use theme::Theme;
pub use views::*;

use crate::app::{App, ViewMode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

/// Render the UI
pub fn render(f: &mut Frame, app: &App) {
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Status bar
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Command line
        ])
        .split(f.size());

    // Render the status bar
    views::status_bar::render(f, app, chunks[0]);

    // Render the main content based on the view mode
    match app.view_mode {
        ViewMode::List => render_list_view(f, app, chunks[1]),
        ViewMode::Detail => views::detail_view::render(f, app, chunks[1]),
        ViewMode::Help => views::help::render(f, app, chunks[1]),
    }

    // Render the command line
    views::command_line::render(f, app, chunks[2]);
}

/// Render the list view
fn render_list_view(f: &mut Frame, app: &App, area: Rect) {
    // Split the area into file browser and log list
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // File browser
            Constraint::Percentage(80), // Log list
        ])
        .split(area);

    // Render the file browser
    views::file_browser::render(f, app, chunks[0]);

    // Render the log list
    views::log_list::render(f, app, chunks[1]);
}
