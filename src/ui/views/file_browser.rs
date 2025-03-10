// File Browser View
//
// This file implements the file browser view that shows the loaded DLT files.

use crate::app::App;
use crate::ui::Theme;
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

/// Render the file browser
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let theme = Theme::default();

    // Create the block
    let block = Block::default()
        .title("Files")
        .borders(Borders::ALL)
        .border_style(theme.border_style())
        .title_style(theme.title_style());

    // Create the list items
    let items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let file_name = file
                .path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            let style = if i == app.current_file_idx {
                theme.selected_style()
            } else {
                Style::default()
            };

            let line = Line::from(vec![Span::styled(file_name.to_string(), style)]);
            ListItem::new(Text::from(line))
        })
        .collect();

    // Create the list
    let list = List::new(items)
        .block(block)
        .highlight_style(theme.selected_style());

    f.render_widget(list, area);
}
