// Help View
//
// This file implements the help view that shows keyboard shortcuts and commands.

use crate::app::App;
use crate::ui::Theme;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Render the help view
pub fn render(f: &mut Frame, _app: &App, area: Rect) {
    let theme = Theme::default();

    // Create the block
    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(theme.border_style())
        .title_style(theme.title_style());

    // Create the help text
    let mut lines = Vec::new();

    // Title
    lines.push(Line::from(vec![Span::styled(
        "DLT Log Viewer - Keyboard Shortcuts",
        theme.title_style(),
    )]));
    lines.push(Line::from(vec![Span::raw("")]));

    // Basic navigation
    lines.push(Line::from(vec![Span::styled(
        "Navigation",
        theme.highlight_style(),
    )]));

    // Add navigation key help lines
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "j, ↓"), theme.highlight_style()),
        Span::raw("Move down".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "k, ↑"), theme.highlight_style()),
        Span::raw("Move up".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "g, Home"), theme.highlight_style()),
        Span::raw("Go to top".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "G, End"), theme.highlight_style()),
        Span::raw("Go to bottom".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "PgUp, Ctrl+b"), theme.highlight_style()),
        Span::raw("Page up".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "PgDn, Ctrl+f"), theme.highlight_style()),
        Span::raw("Page down".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "Tab"), theme.highlight_style()),
        Span::raw("Switch between panes".to_string()),
    ]));
    lines.push(Line::from(vec![Span::raw("")]));

    // View controls
    lines.push(Line::from(vec![Span::styled(
        "View Controls",
        theme.highlight_style(),
    )]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "Enter"), theme.highlight_style()),
        Span::raw("Toggle detail view".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "h, ?"), theme.highlight_style()),
        Span::raw("Show/hide help".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "n, p"), theme.highlight_style()),
        Span::raw("Next/previous file".to_string()),
    ]));
    lines.push(Line::from(vec![Span::raw("")]));

    // Filtering and searching
    lines.push(Line::from(vec![Span::styled(
        "Filtering and Searching",
        theme.highlight_style(),
    )]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "/"), theme.highlight_style()),
        Span::raw("Search".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "n"), theme.highlight_style()),
        Span::raw("Next search result".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "N"), theme.highlight_style()),
        Span::raw("Previous search result".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "f"), theme.highlight_style()),
        Span::raw("Filter mode".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "c"), theme.highlight_style()),
        Span::raw("Clear filters".to_string()),
    ]));
    lines.push(Line::from(vec![Span::raw("")]));

    // Filter commands
    lines.push(Line::from(vec![Span::styled(
        "Filter Commands",
        theme.highlight_style(),
    )]));
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {:<14}", ":filter app=APP"),
            theme.highlight_style(),
        ),
        Span::raw("Filter by application ID".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {:<14}", ":filter ctx=CTX"),
            theme.highlight_style(),
        ),
        Span::raw("Filter by context ID".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {:<14}", ":filter level=LEVEL"),
            theme.highlight_style(),
        ),
        Span::raw("Filter by log level".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {:<14}", ":filter clear"),
            theme.highlight_style(),
        ),
        Span::raw("Clear all filters".to_string()),
    ]));
    lines.push(Line::from(vec![Span::raw("")]));

    // Other commands
    lines.push(Line::from(vec![Span::styled(
        "Other Commands",
        theme.highlight_style(),
    )]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "q, Ctrl+c"), theme.highlight_style()),
        Span::raw("Quit".to_string()),
    ]));
    lines.push(Line::from(vec![
        Span::styled(format!("  {:<14}", "r"), theme.highlight_style()),
        Span::raw("Reload files".to_string()),
    ]));

    // Create the paragraph
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(theme.foreground))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
