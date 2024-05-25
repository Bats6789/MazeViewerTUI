use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(f: &mut Frame, app: &App) {
    let title = Paragraph::new("Hello World! (Press 'Q' to quit)")
        .style(Style::default().fg(Color::White).bg(Color::Black));

    f.render_widget(title, f.size());
}
