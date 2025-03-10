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
                    create_list_item(&msg, i == app.selected_message_idx, &theme)
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
fn create_list_item<'a>(msg: &DltMessage, _selected: bool, theme: &'a Theme) -> ListItem<'a> {
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

    // Create the spans
    let line = Line::from(vec![
        Span::raw(format!("{} ", timestamp)),
        Span::styled(
            format!("{:4} {:4} ", app_id, ctx_id),
            Style::default().fg(theme.title),
        ),
        Span::styled(
            format!("[{:?}] ", log_level.unwrap_or_default()),
            level_style,
        ),
        Span::raw(first_line),
    ]);

    ListItem::new(Text::from(line))
}
