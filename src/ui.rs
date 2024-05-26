use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

mod maze_view;

pub fn ui(f: &mut Frame, _app: &App) {
    let main_layout = Layout::vertical([Constraint::Length(3), Constraint::Min(5)]).split(f.size());

    let button_pannel = main_layout[0];
    let maze_layout = main_layout[1];

    let title = Paragraph::new("Hello World! (Press 'Q' to quit)")
        .style(Style::default().fg(Color::White).bg(Color::Black));

    f.render_widget(title, button_pannel);

    let size = if maze_layout.height> maze_layout.width {
        maze_layout.width
    } else {
        maze_layout.height
    };

    let layout = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Max(size),
        Constraint::Min(0),
    ])
    .split(maze_layout);

    let maze_layout = layout[1];

    let layout = Layout::vertical([Constraint::Length(size / 2), Constraint::Min(0)])
        .split(maze_layout);

    let maze_layout = layout[0];

    let info = format!(
        "maze_layout size = ({}, {})",
        maze_layout.as_size().width,
        maze_layout.as_size().height
    );

    let maze_veiwer = maze_view::new();

    f.render_widget(maze_veiwer, maze_layout);
}
