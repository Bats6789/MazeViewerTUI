use std::fs;

use ratatui::style::Color;

use crate::ui::maze_ui::MazeView;

pub enum CurrentScreen {
    Main,
    Size,
    Speed,
    Algorithm
}

#[derive(PartialEq)]
pub enum SizeSetting {
    Width,
    Height,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub size_setting: SizeSetting,
    pub tmp: usize,
    pub default_color: Color,
    pub highlight_fg: Color,
    pub highlight_bg: Color,
    pub gen_bin: String,
    pub solve_bin: String,
    pub maze: String,
    pub maze_steps: Vec<String>,
    pub has_generated: bool,
    pub maze_veiwer: MazeView,
    width: usize,
    height: usize,
    max_size: usize,
    step: usize,
    speed: usize,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            size_setting: SizeSetting::Width,
            tmp: 2,
            default_color: Color::White,
            highlight_fg: Color::Black,
            highlight_bg: Color::Yellow,
            gen_bin: "".to_string(),
            solve_bin: "".to_string(),
            maze: "".to_string(),
            maze_steps: Vec::with_capacity(0),
            speed: 50,
            has_generated: false,
            maze_veiwer: MazeView::new(),
            width: 2,
            height: 2,
            max_size: 2,
            step: 0,
        }
    }

    pub fn clear_maze(&mut self) {
        let wall_row = String::from("#".repeat(2 * self.width + 1) + "\n");
        let cell_row = String::from("# ".repeat(self.width) + "#\n");
        let mut maze = String::with_capacity((2 * self.width + 2) * (2 * self.height + 1) + 1);

        for _ in 0..self.height {
            maze.push_str(&wall_row);
            maze.push_str(&cell_row);
        }
        maze.push_str(&wall_row);
        maze.pop();

        self.maze = maze;
        self.has_generated = false;
    }

    pub fn load_steps(&mut self, arg: &str) {
        let str = fs::read_to_string(arg).expect("Failed to read: {arg}");

        // replace CRLF with just LF if they exist (only on Windows)
        let str = str.replace("\r\n\r\n", "\n\n");

        self.maze_steps = str.split("\n\n")
            .map(|s| s.to_string())
            .collect();
    }

    pub fn set_width(&mut self, size: usize) {
        self.width = if size < 2 {
            2
        } else if size > self.max_size {
            self.max_size
        } else {
            size
        };
    }

    pub fn set_height(&mut self, size: usize) {
        self.height = if size < 2 {
            2
        } else if size > self.max_size {
            self.max_size
        } else {
            size
        };
    }

    pub fn set_max_size(&mut self, size: usize) {
        if size < 2 {
            self.max_size = 2;
        } else {
            self.max_size = size;
        }
    }

    pub fn set_step_val(&mut self, step: usize) {
        self.step = if step >= self.maze_steps.len() {
            self.maze_steps.len() - 1
        } else {
            step
        };
    }

    pub fn set_speed(&mut self, speed: usize) {
        self.speed = if speed < 1 {
            1
        } else if speed > 100 {
            100
        } else {
            speed
        };
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_max_size(&self) -> usize {
        self.max_size
    }

    pub fn get_step_val(&self) -> usize {
        self.step
    }

    pub fn get_step(&self) -> &String {
        &self.maze_steps[self.step]
    }

    pub fn get_speed(&self) -> usize {
        self.speed
    }

    pub fn get_period(&self) -> u64 {
        u64::try_from(1000 / self.speed).unwrap()
    }
}

#[cfg(test)]
mod app_tests {
    use super::*;

    #[test]
    fn set_height_test() {
        let mut app = App::new();

        app.set_max_size(10);

        assert_eq!(
            app.get_height(),
            2,
            "App.height should start as 2. Got {} instead",
            app.get_height()
        );

        app.set_height(5);

        assert_eq!(
            app.get_height(),
            5,
            "App.height should have changed to 5. Got {} instead",
            app.get_height()
        );

        app.set_height(11);

        assert_eq!(
            app.get_height(),
            10,
            "App.height should not go above 10. Got {} instead",
            app.get_height()
        );
    }

    #[test]
    fn set_width_test() {
        let mut app = App::new();

        app.set_max_size(10);

        assert_eq!(
            app.get_width(),
            2,
            "App.width should start as 2. Got {} instead",
            app.get_width()
        );

        app.set_width(5);

        assert_eq!(
            app.get_width(),
            5,
            "App.width should have changed to 5. Got {} instead",
            app.get_width()
        );

        app.set_width(11);

        assert_eq!(
            app.get_width(),
            10,
            "App.width should not go above 10. Got {} instead",
            app.get_width()
        );
    }

    #[test]
    fn clear_maze_test() {
        let mut app = App::new();

        app.set_max_size(3);
        app.set_width(3);
        app.set_height(3);

        app.clear_maze();

        let expected = "\
#######
# # # #
#######
# # # #
#######
# # # #
#######";

        assert_eq!(expected, app.maze);
    }

    #[test]
    fn load_steps_test() {
        let mut app = App::new();

        
        let test_str = "\
#####
# # #
#####
# # #
#####

#####
# # #
#####
# # #
#####

#####
# # #
#####
# # #
#####";

        let expected = vec![
            "\
#####
# # #
#####
# # #
#####",
"\
#####
# # #
#####
# # #
#####",
"\
#####
# # #
#####
# # #
#####",

        ];

        let _ = fs::write("tmp.steps", test_str);

        app.load_steps("tmp.steps");

        let _ = fs::remove_file("tmp.steps");

        assert_eq!(expected, app.maze_steps, "Maze steps did not parse the maze correctly");
        assert_eq!(3, app.maze_steps.len(), "Maze steps was not 3. Got {}", app.maze_steps.len());
    }
}
