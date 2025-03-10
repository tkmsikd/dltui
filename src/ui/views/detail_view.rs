// Detail View
//
// This file implements the detail view that shows the details of a selected DLT message.

use crate::app::App;
use crate::parser::DltMessage;
use crate::ui::Theme;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Render the detail view
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let theme = Theme::default();

    // Create the block
    let block = Block::default()
        .title("Message Details")
        .borders(Borders::ALL)
        .border_style(theme.border_style())
        .title_style(theme.title_style());

    // Split the area into header and payload
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Header
            Constraint::Min(0),    // Payload
        ])
        .split(block.inner(area));

    // Render the block
    f.render_widget(block, area);

    // Get the selected message
    if let Some(msg) = app.selected_message() {
        // Render the header
        render_header(f, &msg, chunks[0], &theme);

        // Render the payload
        render_payload(f, &msg, chunks[1], &theme);
    } else {
        // No message selected
        let text = Text::from("No message selected");
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(theme.foreground))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[0]);
    }
}

/// Render the message header
fn render_header(f: &mut Frame, msg: &DltMessage, area: Rect, theme: &Theme) {
    let mut lines = Vec::new();

    // Timestamp
    let timestamp = msg.timestamp().format("%Y-%m-%d %H:%M:%S%.6f");
    lines.push(Line::from(vec![
        Span::styled("Timestamp: ", theme.title_style()),
        Span::raw(format!("{}", timestamp)),
    ]));

    // ECU ID
    lines.push(Line::from(vec![
        Span::styled("ECU ID: ", theme.title_style()),
        Span::raw(msg.ecu_id()),
    ]));

    // Application ID
    if let Some(app_id) = msg.app_id() {
        lines.push(Line::from(vec![
            Span::styled("App ID: ", theme.title_style()),
            Span::raw(app_id),
        ]));
    }

    // Context ID
    if let Some(ctx_id) = msg.context_id() {
        lines.push(Line::from(vec![
            Span::styled("Context ID: ", theme.title_style()),
            Span::raw(ctx_id),
        ]));
    }

    // Log level
    if let Some(level) = msg.log_level() {
        lines.push(Line::from(vec![
            Span::styled("Log Level: ", theme.title_style()),
            Span::styled(
                format!("{:?}", level),
                theme.style_for_log_level(Some(level)),
            ),
        ]));
    }

    // Message type
    lines.push(Line::from(vec![
        Span::styled("Message Type: ", theme.title_style()),
        Span::raw(format!("{:?}", msg.message_type())),
    ]));

    // Message counter
    lines.push(Line::from(vec![
        Span::styled("Message Counter: ", theme.title_style()),
        Span::raw(format!("{}", msg.standard_header.message_counter)),
    ]));

    // Render the paragraph
    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(theme.foreground))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

/// Render the message payload
fn render_payload(f: &mut Frame, msg: &DltMessage, area: Rect, theme: &Theme) {
    // Create the block
    let block = Block::default()
        .title("Payload")
        .borders(Borders::ALL)
        .border_style(theme.border_style())
        .title_style(theme.title_style());

    // Get the payload text
    let payload_text = msg.payload_as_text();

    // Create the paragraph
    let paragraph = Paragraph::new(payload_text)
        .style(Style::default().fg(theme.foreground))
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}
