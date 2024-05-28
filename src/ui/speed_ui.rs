use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::app::App;

pub fn speed_ui(f: &mut Frame, speed_layout: Rect, app: &mut App) {
    let layout = Layout::vertical([Constraint::Min(0), Constraint::Max(3), Constraint::Min(0)])
        .split(speed_layout)[1];

    let display_str = format!("Speed: {: >3} steps/s", app.tmp);
    let display_length = u16::try_from(display_str.len()).unwrap();

    let layout = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Max(display_length + 2),
        Constraint::Min(0),
    ])
    .split(layout)[1];

    let display = Paragraph::new(display_str)
        .style(Style::new().fg(app.default_color))
        .centered()
        .block(Block::bordered());

    f.render_widget(display, layout);
}
