// Status Bar View
//
// This file implements the status bar view at the top of the application.

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

/// Render the status bar
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let theme = Theme::default();

    // Create the status text
    let mut status_parts = Vec::new();

    // Add the file info
    if !app.files.is_empty() {
        let file = &app.files[app.current_file_idx];
        let file_name = file
            .path()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        let message_count = file.message_count();
        let filtered_count = app.filtered_messages.len();

        status_parts.push(Span::styled(
            format!(" {} ", file_name),
            theme.title_style(),
        ));

        status_parts.push(Span::raw(" | "));

        status_parts.push(Span::raw(format!(
            "Messages: {}/{} ",
            filtered_count, message_count
        )));
    }

    // Add filter info
    if app.filter.app_id.is_some()
        || app.filter.context_id.is_some()
        || app.filter.log_level.is_some()
        || app.filter.message_type.is_some()
    {
        status_parts.push(Span::raw(" | "));
        status_parts.push(Span::styled("Filtered", theme.highlight_style()));

        if let Some(app_id) = &app.filter.app_id {
            status_parts.push(Span::raw(format!(" App:{}", app_id)));
        }

        if let Some(ctx_id) = &app.filter.context_id {
            status_parts.push(Span::raw(format!(" Ctx:{}", ctx_id)));
        }

        if let Some(level) = &app.filter.log_level {
            status_parts.push(Span::raw(format!(" Level:{:?}", level)));
        }
    }

    // Add search info
    if let Some(_pattern) = &app.search_pattern {
        let result_count = app.search_results.len();
        let current_idx = app.current_search_idx.saturating_add(1).min(result_count);
        let case_mode = if app.case_sensitive_search {
            "Cs"
        } else {
            "Ci"
        };

        status_parts.push(Span::raw(" | "));
        status_parts.push(Span::styled(
            format!("Search[{}]: {}/{}", case_mode, current_idx, result_count),
            theme.highlight_style(),
        ));
    }

    // Add status message if any
    if !app.status_message.is_empty() {
        status_parts.push(Span::raw(" | "));
        status_parts.push(Span::styled(
            app.status_message.clone(),
            Style::default().fg(theme.info),
        ));
    }

    // Create the paragraph
    let status_line = Line::from(status_parts);
    let status_text = Text::from(vec![status_line]);
    let status = Paragraph::new(status_text).style(theme.status_bar_style());

    f.render_widget(status, area);
}
