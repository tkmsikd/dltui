// Command Line View
//
// This file implements the command line view at the bottom of the application.

use crate::app::App;
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
    let line = if !app.command_input.is_empty() {
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
            Span::raw(":help"),
        ])
    };

    // Create the paragraph
    let text = Text::from(vec![line]);
    let command_line = Paragraph::new(text).style(theme.command_line_style());

    f.render_widget(command_line, area);
}
