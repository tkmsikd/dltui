// Log List View
//
// This file implements the log list view that shows the DLT messages.

use crate::app::App;
use crate::parser::DltMessage;
use crate::ui::Theme;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

/// Render the log list
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let theme = Theme::default();

    // Create the block
    let block = Block::default()
        .title("Messages")
        .borders(Borders::ALL)
        .border_style(theme.border_style())
        .title_style(theme.title_style());

    // Create the list items
    let items: Vec<ListItem> = if app.files.is_empty() || app.filtered_messages.is_empty() {
        vec![ListItem::new("No messages")]
    } else {
        let file = &app.files[app.current_file_idx];

        app.filtered_messages
            .iter()
            .enumerate()
            .map(|(i, &idx)| {
                if let Ok(msg) = file.get_message(idx) {
                    // Check if this message is in the search results
                    let is_search_result = app.search_results.contains(&i);

                    create_list_item(
                        &msg,
                        i == app.selected_message_idx,
                        &theme,
                        app.search_pattern.as_ref(),
                        is_search_result,
                    )
                } else {
                    ListItem::new("Error loading message")
                }
            })
            .collect()
    };

    // Create the list state
    let mut state = ListState::default();
    state.select(Some(app.selected_message_idx));

    // Create the list
    let list = List::new(items)
        .block(block)
        .highlight_style(theme.selected_style());

    f.render_stateful_widget(list, area, &mut state);
}

/// Create a list item for a DLT message
fn create_list_item<'a>(
    msg: &DltMessage,
    _selected: bool,
    theme: &'a Theme,
    search_pattern: Option<&regex::Regex>,
    is_search_result: bool,
) -> ListItem<'a> {
    // Format the timestamp
    let timestamp = msg.timestamp().format("%H:%M:%S%.3f");

    // Get the log level and style
    let log_level = msg.log_level();
    let level_style = theme.style_for_log_level(log_level);

    // Format the application and context IDs
    let app_id = msg.app_id().unwrap_or_else(|| "".to_string());
    let ctx_id = msg.context_id().unwrap_or_else(|| "".to_string());

    // Format the payload (first line only)
    let payload = msg.payload_as_text();
    let first_line = payload.lines().next().unwrap_or("").to_string();

    // Create the spans for the prefix
    let mut spans = vec![
        Span::raw(format!("{} ", timestamp)),
        Span::styled(
            format!("{:4} {:4} ", app_id, ctx_id),
            Style::default().fg(theme.title),
        ),
        Span::styled(
            format!("[{:?}] ", log_level.unwrap_or_default()),
            level_style,
        ),
    ];

    // Highlight search matches in the payload if applicable
    if let Some(pattern) = search_pattern {
        let mut last_match_end = 0;
        let mut matches = pattern.find_iter(&first_line).peekable();

        if matches.peek().is_some() {
            // There are matches, add spans with highlighted matches
            for m in pattern.find_iter(&first_line) {
                // Add text before the match
                if m.start() > last_match_end {
                    spans.push(Span::raw(first_line[last_match_end..m.start()].to_string()));
                }

                // Add the highlighted match
                spans.push(Span::styled(
                    first_line[m.start()..m.end()].to_string(),
                    Style::default().fg(theme.highlight),
                ));

                last_match_end = m.end();
            }

            // Add any remaining text after the last match
            if last_match_end < first_line.len() {
                spans.push(Span::raw(first_line[last_match_end..].to_string()));
            }
        } else {
            // No matches, just add the raw text
            spans.push(Span::raw(first_line));
        }
    } else {
        // No search pattern, just add the raw text
        spans.push(Span::raw(first_line));
    }

    // Add a search result indicator if this is a search result
    if is_search_result {
        spans.push(Span::styled(
            " [MATCH]",
            Style::default().fg(theme.highlight),
        ));
    }

    let line = Line::from(spans);

    ListItem::new(Text::from(line))
}
