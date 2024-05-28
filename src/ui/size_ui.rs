use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::app::App;

pub fn size_ui(f: &mut Frame, size_layout: Rect, app: &mut App) {
    let layout =
        Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Min(0)]).split(size_layout);

    let width_layout = layout[0];
    let height_layout = layout[1];
    let default_style = Style::new().fg(app.default_color);
    let highlight_style = Style::new().fg(app.highlight_fg).bg(app.highlight_bg);

    let mut width = app.get_width();
    let mut height = app.get_height();
    let mut width_style = default_style;
    let mut height_style = default_style;

    match app.size_setting {
        crate::app::SizeSetting::Width => {
            width = app.size;
            width_style = highlight_style;
        }
        crate::app::SizeSetting::Height => {
            height = app.size;
            height_style = highlight_style;
        }
    }

    let width_str = format!("Width: {width: >2}");
    let height_str = format!("Height: {height: >2}");

    let width_str_length = u16::try_from(width_str.len()).unwrap();
    let height_str_length = u16::try_from(height_str.len()).unwrap();

    let width_layout =
        Layout::vertical([Constraint::Min(0), Constraint::Max(3), Constraint::Min(0)])
            .split(width_layout)[1];
    let height_layout =
        Layout::vertical([Constraint::Min(0), Constraint::Max(3), Constraint::Min(0)])
            .split(height_layout)[1];

    let width_layout = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Max(width_str_length + 2),
        Constraint::Min(0),
    ])
    .split(width_layout)[1];
    let height_layout = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Max(height_str_length + 2),
        Constraint::Min(0),
    ])
    .split(height_layout)[1];

    let width_display = Paragraph::new(width_str)
        .style(width_style)
        .centered()
        .block(Block::bordered());
    let height_display = Paragraph::new(height_str)
        .style(height_style)
        .centered()
        .block(Block::bordered());

    f.render_widget(width_display, width_layout);
    f.render_widget(height_display, height_layout);
}
