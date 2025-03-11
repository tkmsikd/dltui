// Command Line View
//
// This file implements the command line view at the bottom of the application.

use crate::app::App;
use crate::app::InputMode;
use crate::ui::Theme;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

/// Render the command line
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let theme = Theme::default();

    // Create the command line text
    let line = match app.input_mode {
        InputMode::Search => {
            // Show search input
            Line::from(vec![
                Span::styled("/", Style::default().fg(theme.highlight)),
                Span::raw(&app.command_input),
            ])
        }
        InputMode::Filter => {
            // Show filter input
            Line::from(vec![
                Span::styled(":", Style::default().fg(theme.highlight)),
                Span::raw("filter "),
                Span::raw(&app.command_input),
            ])
        }
        InputMode::Normal => {
            if !app.command_input.is_empty() {
                // Show the command being typed
                Line::from(vec![
                    Span::styled(":", Style::default().fg(theme.highlight)),
                    Span::raw(&app.command_input),
                ])
            } else {
                // Show help text
                Line::from(vec![
                    Span::styled("q", Style::default().fg(theme.highlight)),
                    Span::raw(":quit "),
                    Span::styled("/", Style::default().fg(theme.highlight)),
                    Span::raw(":search "),
                    Span::styled("f", Style::default().fg(theme.highlight)),
                    Span::raw(":filter "),
                    Span::styled("h", Style::default().fg(theme.highlight)),
                    Span::raw(":help "),
                    Span::styled("n", Style::default().fg(theme.highlight)),
                    Span::raw(":next "),
                    Span::styled("N", Style::default().fg(theme.highlight)),
                    Span::raw(":prev "),
                    Span::styled("i", Style::default().fg(theme.highlight)),
                    Span::raw(":case"),
                ])
            }
        }
    };

    // Create the paragraph
    let text = Text::from(vec![line]);
    let command_line = Paragraph::new(text).style(theme.command_line_style());

    f.render_widget(command_line, area);
}
