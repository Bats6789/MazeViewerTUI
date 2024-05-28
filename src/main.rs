use crate::ui::ui;

use std::{error::Error, fs, io, process::Command, time::Duration};

use app::{App, CurrentScreen, SizeSetting};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use color_eyre::Result;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    match std::env::var("MAZE_GEN") {
        Ok(str) => app.gen_bin = str,
        Err(err) => {
            disable_raw_mode()?;

            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;

            eprintln!("\"MAZE_GEN\" must be defined to the path of the maze generator.");

            return Err(Box::new(err));
        }
    };

    match std::env::var("MAZE_SOLVE") {
        Ok(str) => app.solve_bin = str,
        Err(err) => {
            disable_raw_mode()?;

            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;

            eprintln!("\"MAZE_SOLVE\" must be defined to the path of the maze generator.");

            return Err(Box::new(err));
        }
    };

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(..) = res {
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    terminal.draw(|f| ui(f, app))?;

    app.set_width(app.get_max_size() / 2);
    app.set_height(app.get_max_size() / 2);

    app.clear_maze();

    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(());
                    }
                    KeyCode::Char('z') | KeyCode::Char('Z') => {
                        app.current_screen = CurrentScreen::Size;
                        app.size = match app.size_setting {
                            SizeSetting::Width => app.get_width(),
                            SizeSetting::Height => app.get_height(),
                        };
                    }
                    KeyCode::Char('g') | KeyCode::Char('G') => {
                        let output = Command::new(&app.gen_bin)
                            .args([
                                "-q",
                                "-v",
                                "maze.steps",
                                &app.get_width().to_string(),
                                &app.get_height().to_string(),
                            ])
                            .output()
                            .expect("Failed to call maze generator.");
                        app.maze = String::from_utf8(output.stdout).unwrap();
                        app.load_steps("maze.steps");
                        app.set_step_val(0);
                        app.has_generated = true;
                        let _ = fs::write("maze.mz", &app.maze);
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        if !app.has_generated {
                            continue;
                        }
                        let output = Command::new(&app.solve_bin)
                            .args([
                                "-q",
                                "-v",
                                "maze.steps",
                                "-i",
                                "maze.mz",
                                &app.get_width().to_string(),
                                &app.get_height().to_string(),
                            ])
                            .output()
                            .expect("Failed to call maze solver.");
                        app.maze = String::from_utf8(output.stdout).unwrap();
                        app.load_steps("maze.steps");
                        app.set_step_val(0);
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        if app.get_step_val() == app.maze_steps.len() - 1 {
                            app.set_step_val(0);
                        }
                        let start = app.get_step_val();
                        for i in start..app.maze_steps.len() {
                            app.set_step_val(i);
                            app.maze = app.get_step().clone();
                            std::thread::sleep(Duration::from_millis(app.get_period()));
                            terminal.draw(|f| ui(f, app))?;
                        }
                    }
                    KeyCode::Left => {
                        if app.has_generated && app.get_step_val() > 0 {
                            app.set_step_val(app.get_step_val() - 1);
                            app.maze = app.get_step().clone();
                        }
                    }
                    KeyCode::Right => {
                        if app.has_generated && app.get_step_val() < app.maze_steps.len() - 1 {
                            app.set_step_val(app.get_step_val() + 1);
                            app.maze = app.get_step().clone();
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Size => match key.code {
                    KeyCode::Esc => {
                        match app.size_setting {
                            SizeSetting::Width => app.set_width(app.size),
                            SizeSetting::Height => app.set_height(app.size),
                        }
                        app.clear_maze();
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        if app.size_setting == SizeSetting::Height {
                            app.set_height(app.size);
                        }
                        app.size_setting = SizeSetting::Width;
                        app.size = app.get_width();
                    }
                    KeyCode::Char('h') | KeyCode::Char('H') => {
                        if app.size_setting == SizeSetting::Width {
                            app.set_width(app.size);
                        }
                        app.size_setting = SizeSetting::Height;
                        app.size = app.get_height();
                    }
                    KeyCode::Backspace => {
                        if app.size < 10 {
                            app.size = 0;
                        } else {
                            app.size /= 10;
                        }
                    }
                    KeyCode::Char('0') => {
                        app.size = enter_value(0, app.size, app.get_max_size());
                    }
                    KeyCode::Char('1') => {
                        app.size = enter_value(1, app.size, app.get_max_size());
                    }
                    KeyCode::Char('2') => {
                        app.size = enter_value(2, app.size, app.get_max_size());
                    }
                    KeyCode::Char('3') => {
                        app.size = enter_value(3, app.size, app.get_max_size());
                    }
                    KeyCode::Char('4') => {
                        app.size = enter_value(4, app.size, app.get_max_size());
                    }
                    KeyCode::Char('5') => {
                        app.size = enter_value(5, app.size, app.get_max_size());
                    }
                    KeyCode::Char('6') => {
                        app.size = enter_value(6, app.size, app.get_max_size());
                    }
                    KeyCode::Char('7') => {
                        app.size = enter_value(7, app.size, app.get_max_size());
                    }
                    KeyCode::Char('8') => {
                        app.size = enter_value(8, app.size, app.get_max_size());
                    }
                    KeyCode::Char('9') => {
                        app.size = enter_value(9, app.size, app.get_max_size());
                    }
                    KeyCode::Enter => match app.size_setting {
                        SizeSetting::Width => app.set_width(app.size),
                        SizeSetting::Height => app.set_height(app.size),
                    },
                    _ => {}
                },
            }
        }
    }
}

fn enter_value(val: usize, current_val: usize, max_val: usize) -> usize {
    let value = current_val * 10 + val;

    if value > max_val {
        val
    } else {
        value
    }
}
