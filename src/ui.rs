pub mod maze_ui;
mod size_ui;
mod speed_ui;

use ratatui::{
    layout::{Constraint, Layout},
    style::Style,
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, CurrentScreen};

use self::{maze_ui::maze_ui, size_ui::size_ui, speed_ui::speed_ui};

pub fn ui(f: &mut Frame, app: &mut App) {
    let main_layout = Layout::vertical([Constraint::Length(3), Constraint::Min(5)]).split(f.size());

    let button_pannel = main_layout[0];
    let display_pannel = main_layout[1];

    let size = if display_pannel.height > display_pannel.width {
        display_pannel.width
    } else {
        display_pannel.height
    };

    app.set_max_size(usize::from((size - 1) / 2));

    if app.maze == "" {
        app.clear_maze();
    }

    let text = match app.current_screen {
        CurrentScreen::Main => {
            let mut opts = "\nQuit: Q | Size settings: Z | Speed setting: P | Generate: G".to_string();
            if app.has_generated {
                opts += &format!(
                    " | Solve: S\nRun: R | Next step: Right | Previous step: Left | Step {}/{}",
                    app.get_step_val(),
                    if !app.has_generated {
                        0
                    } else {
                        app.maze_steps.len() - 1
                    }
                )
                .to_string();
            }

            opts
        }
        CurrentScreen::Size => {
            "\nExit: Esc | Enter value: Enter | Width: W | Height: H".to_string()
        }
        CurrentScreen::Speed => "\nExit: Esc | Enter value: Enter".to_string(),
        CurrentScreen::Algorithm => todo!(),
    };

    let keybind_hints = Paragraph::new(text)
        .style(Style::default().fg(app.default_color))
        .centered();

    f.render_widget(keybind_hints, button_pannel);

    match app.current_screen {
        CurrentScreen::Main => maze_ui(f, display_pannel, app),
        CurrentScreen::Size => size_ui(f, display_pannel, app),
        CurrentScreen::Speed => speed_ui(f, display_pannel, app),
        CurrentScreen::Algorithm => todo!(),
    };
}
