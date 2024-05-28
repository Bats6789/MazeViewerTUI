use crate::{app::AlgorithmSetting, ui::ui};

use std::{error::Error, fs, io, process::Command, time::Duration};

use app::{App, BiasMethods, CurrentScreen, GenAlgorithms, SizeSetting, TreeSubAlgorithm};
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
                        app.tmp = match app.size_setting {
                            SizeSetting::Width => app.get_width(),
                            SizeSetting::Height => app.get_height(),
                        };
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        app.tmp = app.get_speed();
                        app.current_screen = CurrentScreen::Speed;
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        app.current_screen = CurrentScreen::Algorithm;
                    }
                    KeyCode::Char('g') | KeyCode::Char('G') => {
                        let mut args =
                            Vec::from(["-q", "-v", "maze.steps", "-a"].map(|s| s.to_string()));

                        for arg in app.gen_algorithm.to_string().split(' ') {
                            args.push(arg.to_string());
                        }

                        args.push(app.get_width().to_string());
                        args.push(app.get_height().to_string());

                        let output = Command::new(&app.gen_bin)
                            .args(args)
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

                        let mut args =
                            Vec::from(["-q", "-v", "maze.steps", "-i", "maze.mz", "-a"].map(|s| s.to_string()));

                        for arg in app.solve_algorithm.to_string().split(' ') {
                            args.push(arg.to_string());
                        }

                        args.push(app.get_width().to_string());
                        args.push(app.get_height().to_string());

                        let output = Command::new(&app.solve_bin)
                            .args(args)
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
                            SizeSetting::Width => app.set_width(app.tmp),
                            SizeSetting::Height => app.set_height(app.tmp),
                        }
                        app.clear_maze();
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        if app.size_setting == SizeSetting::Height {
                            app.set_height(app.tmp);
                        }
                        app.size_setting = SizeSetting::Width;
                        app.tmp = app.get_width();
                    }
                    KeyCode::Char('h') | KeyCode::Char('H') => {
                        if app.size_setting == SizeSetting::Width {
                            app.set_width(app.tmp);
                        }
                        app.size_setting = SizeSetting::Height;
                        app.tmp = app.get_height();
                    }
                    KeyCode::Backspace => {
                        if app.tmp < 10 {
                            app.tmp = 0;
                        } else {
                            app.tmp /= 10;
                        }
                    }
                    KeyCode::Char('0') => {
                        app.tmp = enter_value(0, app.tmp, app.get_max_size());
                    }
                    KeyCode::Char('1') => {
                        app.tmp = enter_value(1, app.tmp, app.get_max_size());
                    }
                    KeyCode::Char('2') => {
                        app.tmp = enter_value(2, app.tmp, app.get_max_size());
                    }
                    KeyCode::Char('3') => {
                        app.tmp = enter_value(3, app.tmp, app.get_max_size());
                    }
                    KeyCode::Char('4') => {
                        app.tmp = enter_value(4, app.tmp, app.get_max_size());
                    }
                    KeyCode::Char('5') => {
                        app.tmp = enter_value(5, app.tmp, app.get_max_size());
                    }
                    KeyCode::Char('6') => {
                        app.tmp = enter_value(6, app.tmp, app.get_max_size());
                    }
                    KeyCode::Char('7') => {
                        app.tmp = enter_value(7, app.tmp, app.get_max_size());
                    }
                    KeyCode::Char('8') => {
                        app.tmp = enter_value(8, app.tmp, app.get_max_size());
                    }
                    KeyCode::Char('9') => {
                        app.tmp = enter_value(9, app.tmp, app.get_max_size());
                    }
                    KeyCode::Enter => match app.size_setting {
                        SizeSetting::Width => app.set_width(app.tmp),
                        SizeSetting::Height => app.set_height(app.tmp),
                    },
                    _ => {}
                },
                CurrentScreen::Speed => match key.code {
                    KeyCode::Esc => {
                        app.set_speed(app.tmp);
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char('0') => {
                        app.tmp = enter_value(0, app.tmp, 100);
                    }
                    KeyCode::Char('1') => {
                        app.tmp = enter_value(1, app.tmp, 100);
                    }
                    KeyCode::Char('2') => {
                        app.tmp = enter_value(2, app.tmp, 100);
                    }
                    KeyCode::Char('3') => {
                        app.tmp = enter_value(3, app.tmp, 100);
                    }
                    KeyCode::Char('4') => {
                        app.tmp = enter_value(4, app.tmp, 100);
                    }
                    KeyCode::Char('5') => {
                        app.tmp = enter_value(5, app.tmp, 100);
                    }
                    KeyCode::Char('6') => {
                        app.tmp = enter_value(6, app.tmp, 100);
                    }
                    KeyCode::Char('7') => {
                        app.tmp = enter_value(7, app.tmp, 100);
                    }
                    KeyCode::Char('8') => {
                        app.tmp = enter_value(8, app.tmp, 100);
                    }
                    KeyCode::Char('9') => {
                        app.tmp = enter_value(9, app.tmp, 100);
                    }
                    KeyCode::Enter => {
                        app.set_speed(app.tmp);
                        app.tmp = app.get_speed();
                    }
                    _ => {}
                },
                CurrentScreen::Algorithm => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Tab => match app.algorithm_setting {
                        AlgorithmSetting::Generator => {
                            app.algorithm_setting = AlgorithmSetting::Solver
                        }
                        AlgorithmSetting::Solver => {
                            app.algorithm_setting = AlgorithmSetting::Generator
                        }
                    },
                    KeyCode::Down => match app.algorithm_setting {
                        AlgorithmSetting::Generator => {
                            let new_state = app.gen_list_state.selected().unwrap() + 1;
                            if new_state < app.gen_algo_lookup.len() {
                                app.gen_list_state.select(Some(new_state));
                                app.gen_algorithm = app.gen_algo_lookup[new_state].clone();
                            }
                        }
                        AlgorithmSetting::Solver => {
                            let new_state = app.solve_list_state.selected().unwrap() + 1;
                            if new_state < app.solve_algo_lookup.len() {
                                app.solve_list_state.select(Some(new_state));
                                app.solve_algorithm = app.solve_algo_lookup[new_state].clone();
                            }
                        }
                    },
                    KeyCode::Up => match app.algorithm_setting {
                        AlgorithmSetting::Generator => {
                            let mut new_state = app.gen_list_state.selected().unwrap();
                            if new_state > 0 {
                                new_state -= 1;
                                app.gen_list_state.select(Some(new_state));
                                app.gen_algorithm = app.gen_algo_lookup[new_state].clone();
                            }
                        }
                        AlgorithmSetting::Solver => {
                            let mut new_state = app.solve_list_state.selected().unwrap();
                            if new_state > 0 {
                                new_state -= 1;
                                app.solve_list_state.select(Some(new_state));
                                app.solve_algorithm = app.solve_algo_lookup[new_state].clone();
                            }
                        }
                    },
                    KeyCode::Right => {
                        if app.algorithm_setting == AlgorithmSetting::Generator {
                            if let GenAlgorithms::GrowingTree(method) = &app.gen_algorithm {
                                app.gen_algorithm = match method {
                                    TreeSubAlgorithm::Newest => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::Middle)
                                    }
                                    TreeSubAlgorithm::Middle => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::Oldest)
                                    }
                                    TreeSubAlgorithm::Oldest => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::Random)
                                    }
                                    TreeSubAlgorithm::Random => GenAlgorithms::GrowingTree(
                                        TreeSubAlgorithm::NewestMiddle(app.get_ratio()),
                                    ),
                                    TreeSubAlgorithm::NewestMiddle(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::NewestOldest(
                                            app.get_ratio(),
                                        ))
                                    }
                                    TreeSubAlgorithm::NewestOldest(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::NewestRandom(
                                            app.get_ratio(),
                                        ))
                                    }
                                    TreeSubAlgorithm::NewestRandom(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::MiddleOldest(
                                            app.get_ratio(),
                                        ))
                                    }
                                    TreeSubAlgorithm::MiddleOldest(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::MiddleRandom(
                                            app.get_ratio(),
                                        ))
                                    }
                                    TreeSubAlgorithm::MiddleRandom(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::OldestRandom(
                                            app.get_ratio(),
                                        ))
                                    }
                                    TreeSubAlgorithm::OldestRandom(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::Newest)
                                    }
                                };
                                app.gen_algo_lookup[app.gen_list_state.selected().unwrap()] =
                                    app.gen_algorithm.clone();
                            } else if let GenAlgorithms::BinaryTree(bias) = &app.gen_algorithm {
                                app.gen_algorithm = match bias {
                                    BiasMethods::NorthWest => {
                                        GenAlgorithms::BinaryTree(BiasMethods::NorthEast)
                                    }
                                    BiasMethods::NorthEast => {
                                        GenAlgorithms::BinaryTree(BiasMethods::SouthWest)
                                    }
                                    BiasMethods::SouthWest => {
                                        GenAlgorithms::BinaryTree(BiasMethods::SouthEast)
                                    }
                                    BiasMethods::SouthEast => {
                                        GenAlgorithms::BinaryTree(BiasMethods::NorthWest)
                                    }
                                };

                                app.gen_algo_lookup[app.gen_list_state.selected().unwrap()] =
                                    app.gen_algorithm.clone();
                            }
                        }
                    }
                    KeyCode::Left => {
                        if app.algorithm_setting == AlgorithmSetting::Generator {
                            if let GenAlgorithms::GrowingTree(method) = &app.gen_algorithm {
                                app.gen_algorithm = match method {
                                    TreeSubAlgorithm::Newest => GenAlgorithms::GrowingTree(
                                        TreeSubAlgorithm::OldestRandom(app.get_ratio()),
                                    ),
                                    TreeSubAlgorithm::Middle => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::Newest)
                                    }
                                    TreeSubAlgorithm::Oldest => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::Middle)
                                    }
                                    TreeSubAlgorithm::Random => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::Oldest)
                                    }
                                    TreeSubAlgorithm::NewestMiddle(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::Random)
                                    }
                                    TreeSubAlgorithm::NewestOldest(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::NewestMiddle(
                                            app.get_ratio(),
                                        ))
                                    }
                                    TreeSubAlgorithm::NewestRandom(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::NewestOldest(
                                            app.get_ratio(),
                                        ))
                                    }
                                    TreeSubAlgorithm::MiddleOldest(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::NewestRandom(
                                            app.get_ratio(),
                                        ))
                                    }
                                    TreeSubAlgorithm::MiddleRandom(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::MiddleOldest(
                                            app.get_ratio(),
                                        ))
                                    }
                                    TreeSubAlgorithm::OldestRandom(_) => {
                                        GenAlgorithms::GrowingTree(TreeSubAlgorithm::MiddleRandom(
                                            app.get_ratio(),
                                        ))
                                    }
                                };
                                app.gen_algo_lookup[app.gen_list_state.selected().unwrap()] =
                                    app.gen_algorithm.clone();
                            } else if let GenAlgorithms::BinaryTree(bias) = &app.gen_algorithm {
                                app.gen_algorithm = match bias {
                                    BiasMethods::NorthWest => {
                                        GenAlgorithms::BinaryTree(BiasMethods::SouthEast)
                                    }
                                    BiasMethods::NorthEast => {
                                        GenAlgorithms::BinaryTree(BiasMethods::NorthWest)
                                    }
                                    BiasMethods::SouthWest => {
                                        GenAlgorithms::BinaryTree(BiasMethods::NorthEast)
                                    }
                                    BiasMethods::SouthEast => {
                                        GenAlgorithms::BinaryTree(BiasMethods::SouthWest)
                                    }
                                };
                                app.gen_algo_lookup[app.gen_list_state.selected().unwrap()] =
                                    app.gen_algorithm.clone();
                            }
                        }
                    }
                    KeyCode::Char('0') => {
                        update_ratio(0, app);
                    }
                    KeyCode::Char('1') => {
                        update_ratio(1, app);
                    }
                    KeyCode::Char('2') => {
                        update_ratio(2, app);
                    }
                    KeyCode::Char('3') => {
                        update_ratio(3, app);
                    }
                    KeyCode::Char('4') => {
                        update_ratio(4, app);
                    }
                    KeyCode::Char('5') => {
                        update_ratio(5, app);
                    }
                    KeyCode::Char('6') => {
                        update_ratio(6, app);
                    }
                    KeyCode::Char('7') => {
                        update_ratio(7, app);
                    }
                    KeyCode::Char('8') => {
                        update_ratio(8, app);
                    }
                    KeyCode::Char('9') => {
                        update_ratio(9, app);
                    }
                    KeyCode::Backspace => {
                        if let GenAlgorithms::GrowingTree(method) = app.gen_algorithm.clone() {
                            let mut ratio = match method {
                                TreeSubAlgorithm::NewestMiddle(ratio) => ratio,
                                TreeSubAlgorithm::NewestOldest(ratio) => ratio,
                                TreeSubAlgorithm::NewestRandom(ratio) => ratio,
                                TreeSubAlgorithm::MiddleOldest(ratio) => ratio,
                                TreeSubAlgorithm::MiddleRandom(ratio) => ratio,
                                TreeSubAlgorithm::OldestRandom(ratio) => ratio,
                                _ => continue,
                            };

                            ratio /= 10.0;
                            app.set_ratio(ratio);

                            app.gen_algorithm = match method {
                                TreeSubAlgorithm::NewestMiddle(_) => GenAlgorithms::GrowingTree(
                                    TreeSubAlgorithm::NewestMiddle(app.get_ratio()),
                                ),
                                TreeSubAlgorithm::NewestOldest(_) => GenAlgorithms::GrowingTree(
                                    TreeSubAlgorithm::NewestOldest(app.get_ratio()),
                                ),
                                TreeSubAlgorithm::NewestRandom(_) => GenAlgorithms::GrowingTree(
                                    TreeSubAlgorithm::NewestRandom(app.get_ratio()),
                                ),
                                TreeSubAlgorithm::MiddleOldest(_) => GenAlgorithms::GrowingTree(
                                    TreeSubAlgorithm::MiddleOldest(app.get_ratio()),
                                ),
                                TreeSubAlgorithm::MiddleRandom(_) => GenAlgorithms::GrowingTree(
                                    TreeSubAlgorithm::MiddleRandom(app.get_ratio()),
                                ),
                                TreeSubAlgorithm::OldestRandom(_) => GenAlgorithms::GrowingTree(
                                    TreeSubAlgorithm::OldestRandom(app.get_ratio()),
                                ),
                                _ => continue,
                            };

                            app.gen_algo_lookup[app.gen_list_state.selected().unwrap()] =
                                app.gen_algorithm.clone();
                        }
                    }
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

fn update_ratio(val: usize, app: &mut App) {
    if let GenAlgorithms::GrowingTree(method) = app.gen_algorithm.clone() {
        app.tmp = match method {
            TreeSubAlgorithm::NewestMiddle(ratio) => (100.0 * ratio) as usize,
            TreeSubAlgorithm::NewestOldest(ratio) => (100.0 * ratio) as usize,
            TreeSubAlgorithm::NewestRandom(ratio) => (100.0 * ratio) as usize,
            TreeSubAlgorithm::MiddleOldest(ratio) => (100.0 * ratio) as usize,
            TreeSubAlgorithm::MiddleRandom(ratio) => (100.0 * ratio) as usize,
            TreeSubAlgorithm::OldestRandom(ratio) => (100.0 * ratio) as usize,
            _ => return,
        };

        app.tmp = enter_value(val, app.tmp, 100);
        let num = app.tmp as f64;
        app.set_ratio(num / 100.0);

        app.gen_algorithm = match method {
            TreeSubAlgorithm::NewestMiddle(_) => {
                GenAlgorithms::GrowingTree(TreeSubAlgorithm::NewestMiddle(app.get_ratio()))
            }
            TreeSubAlgorithm::NewestOldest(_) => {
                GenAlgorithms::GrowingTree(TreeSubAlgorithm::NewestOldest(app.get_ratio()))
            }
            TreeSubAlgorithm::NewestRandom(_) => {
                GenAlgorithms::GrowingTree(TreeSubAlgorithm::NewestRandom(app.get_ratio()))
            }
            TreeSubAlgorithm::MiddleOldest(_) => {
                GenAlgorithms::GrowingTree(TreeSubAlgorithm::MiddleOldest(app.get_ratio()))
            }
            TreeSubAlgorithm::MiddleRandom(_) => {
                GenAlgorithms::GrowingTree(TreeSubAlgorithm::MiddleRandom(app.get_ratio()))
            }
            TreeSubAlgorithm::OldestRandom(_) => {
                GenAlgorithms::GrowingTree(TreeSubAlgorithm::OldestRandom(app.get_ratio()))
            }
            _ => return,
        };

        app.gen_algo_lookup[app.gen_list_state.selected().unwrap()] = app.gen_algorithm.clone();
    }
}
