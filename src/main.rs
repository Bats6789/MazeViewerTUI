use crate::ui::ui;

use std::{error::Error, io};

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
                    _ => {}
                },
                CurrentScreen::Size => match key.code {
                    KeyCode::Esc => {
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
