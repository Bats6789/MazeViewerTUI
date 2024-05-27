use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, CurrentScreen};

use self::maze_view::MazeView;

mod maze_view;

pub fn ui(f: &mut Frame, app: &mut App) {
    let main_layout = Layout::vertical([Constraint::Length(3), Constraint::Min(5)]).split(f.size());

    let button_pannel = main_layout[0];
    let display_pannel = main_layout[1];

    let size = if display_pannel.height> display_pannel.width {
        display_pannel.width
    } else {
        display_pannel.height
    };

    app.set_max_size(((size - 1) / 2).into());

    let title = Paragraph::new("Hello World! (Press 'Q' to quit)")
        .style(Style::default().fg(Color::White).bg(Color::Black));

    f.render_widget(title, button_pannel);

    match app.current_screen {
        CurrentScreen::Main => maze_ui(f, display_pannel),
        CurrentScreen::Size => size_ui(f, display_pannel)
    };
}

fn maze_ui(f: &mut Frame, maze_layout: Rect) {
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

    let layout = Layout::vertical([Constraint::Length(size), Constraint::Min(0)])
        .split(maze_layout);

    let maze_layout = layout[0];

    let mut maze_veiwer = MazeView::new();

    maze_veiwer.load_maze("\
#######
#    X#
# #####
#   # #
### # #
#S    #
#######");

    f.render_widget(maze_veiwer, maze_layout);
}

fn size_ui(f: &mut Frame, size_layout: Rect) {
    let size = if size_layout.height> size_layout.width {
        size_layout.width
    } else {
        size_layout.height
    };

    let layout = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Max(size),
        Constraint::Min(0),
    ])
    .split(size_layout);

    let size_layout = layout[1];

    let layout = Layout::vertical([Constraint::Length(size), Constraint::Min(0)])
        .split(size_layout);

    let maze_layout = layout[0];

    let mut maze_veiwer = MazeView::new();

    maze_veiwer.load_maze("\
#######
#    X#
# #####
#   # #
### # #
#S    #
#######");

    f.render_widget(maze_veiwer, maze_layout);
}
