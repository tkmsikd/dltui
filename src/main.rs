//! DLT Log Viewer
//!
//! A TUI tool for viewing and analyzing Covesa DLT log files.

mod app;
mod config;
mod filter;
mod parser;
mod search;
mod ui;

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::{App, InputMode};
use crate::config::Settings;
use crate::filter::FilterCriteria;
use crate::ui::{Event, EventHandler};

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// DLT files to open
    #[clap(name = "FILE")]
    files: Vec<PathBuf>,

    /// Filter to apply
    #[clap(short, long)]
    filter: Option<String>,

    /// Search pattern
    #[clap(short, long)]
    search: Option<String>,

    /// Config file
    #[clap(short, long)]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Load settings
    let settings = if let Some(config_path) = args.config {
        Settings::load(config_path).unwrap_or_default()
    } else {
        Settings::load_default()
    };

    // Setup terminal
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    // Create app state
    let mut app = App::new();

    // Load files
    for path in &args.files {
        if let Err(e) = app.load_file(path.clone()) {
            eprintln!("Error loading file {}: {}", path.display(), e);
        }
    }

    // Apply filter if specified
    if let Some(filter_str) = args.filter {
        if let Err(e) = app.apply_text_filter(&filter_str) {
            eprintln!("Error applying filter pattern: {}", e);
        }
    }

    // Apply search if specified
    if let Some(search_str) = args.search {
        if let Err(e) = app.search(&search_str) {
            eprintln!("Error applying search pattern: {}", e);
        }
    }

    // Create event handler
    let tick_rate = Duration::from_millis(settings.tick_rate);
    let event_handler = EventHandler::new(tick_rate);

    // Run the main loop
    run_app(&mut terminal, app, event_handler)?;

    // Restore terminal
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    Ok(())
}

/// Run the application
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    event_handler: EventHandler,
) -> Result<()> {
    loop {
        // Draw the UI
        terminal.draw(|f| ui::render(f, &app))?;

        // Handle events
        match event_handler.next()? {
            Event::Key(key) => {
                // Handle keys based on input mode
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        // Quit
                        KeyCode::Char('q') => {
                            app.exit();
                        }
                        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                            app.exit();
                        }

                        // Navigation
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.move_up();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.move_down();
                        }
                        KeyCode::Home | KeyCode::Char('g') => {
                            app.move_to_top();
                        }
                        KeyCode::End | KeyCode::Char('G') => {
                            app.move_to_bottom();
                        }

                        // View controls
                        KeyCode::Enter => {
                            app.toggle_view_mode();
                        }
                        KeyCode::Char('h') | KeyCode::Char('?') => {
                            app.show_help();
                        }

                        // File navigation
                        KeyCode::Char('p') => {
                            app.prev_file();
                        }

                        // Search
                        KeyCode::Char('/') => {
                            app.enter_search_mode();
                        }
                        KeyCode::Char('n') => {
                            app.next_search_result();
                        }
                        KeyCode::Char('N') => {
                            app.prev_search_result();
                        }

                        // Filter
                        KeyCode::Char('f') => {
                            app.enter_filter_mode();
                        }

                        // Toggle case sensitivity for search
                        KeyCode::Char('i') => {
                            if let Err(e) = app.toggle_case_sensitivity() {
                                app.status_message =
                                    format!("Error toggling case sensitivity: {}", e);
                            }
                        }

                        // Other keys
                        _ => {}
                    },
                    InputMode::Search => {
                        // Handle search input
                        if let KeyCode::Char(c) = key.code {
                            app.handle_search_input(c);
                        } else {
                            match key.code {
                                KeyCode::Enter => app.handle_search_input('\n'),
                                KeyCode::Backspace => app.handle_search_input('\u{8}'),
                                KeyCode::Esc => app.handle_search_input('\u{1b}'),
                                _ => {}
                            }
                        }
                    }
                    InputMode::Filter => {
                        // Handle filter input
                        if let KeyCode::Char(c) = key.code {
                            app.handle_filter_input(c);
                        } else {
                            match key.code {
                                KeyCode::Enter => app.handle_filter_input('\n'),
                                KeyCode::Backspace => app.handle_filter_input('\u{8}'),
                                KeyCode::Esc => app.handle_filter_input('\u{1b}'),
                                _ => {}
                            }
                        }
                    }
                }
            }
            Event::Resize(_, _) => {}
            Event::Tick => {}
        }

        // Check if we should exit
        if app.should_exit {
            break;
        }
    }

    Ok(())
}
