use std::fs;

use ratatui::{style::Color, widgets::ListState};

use crate::ui::maze_ui::MazeView;

pub enum CurrentScreen {
    Main,
    Size,
    Speed,
    Algorithm,
}

#[derive(PartialEq, Clone)]
pub enum SizeSetting {
    Width,
    Height,
}

#[derive(PartialEq, Clone)]
pub enum TreeSubAlgorithm {
    Newest,
    Middle,
    Oldest,
    Random,
    NewestMiddle(f64),
    NewestOldest(f64),
    NewestRandom(f64),
    MiddleOldest(f64),
    MiddleRandom(f64),
    OldestRandom(f64),
}

#[derive(PartialEq, Clone)]
pub enum BiasMethods {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

#[derive(PartialEq, Clone)]
pub enum GenAlgorithms {
    Kruskal,
    Prim,
    Back,
    AldousBroder,
    GrowingTree(TreeSubAlgorithm),
    HuntAndKill,
    Wilson,
    Eller,
    Divide,
    Sidewinder,
    BinaryTree(BiasMethods),
}

#[derive(PartialEq, Clone)]
pub enum SolveAlgorithms {
    Depth,
    Breadth,
    Dijkstra,
    AStar,
}

#[derive(PartialEq, Clone)]
pub enum AlgorithmSetting {
    Generator,
    Solver,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub size_setting: SizeSetting,
    pub algorithm_setting: AlgorithmSetting,
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
    pub gen_algorithm: GenAlgorithms,
    pub solve_algorithm: SolveAlgorithms,
    pub gen_list_state: ListState,
    pub solve_list_state: ListState,
    pub gen_algo_lookup: Vec<GenAlgorithms>,
    pub solve_algo_lookup: Vec<SolveAlgorithms>,
    width: usize,
    height: usize,
    max_size: usize,
    ratio: f64,
    step: usize,
    speed: usize,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            size_setting: SizeSetting::Width,
            algorithm_setting: AlgorithmSetting::Generator,
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
            gen_algorithm: GenAlgorithms::Kruskal,
            solve_algorithm: SolveAlgorithms::Depth,
            gen_list_state: ListState::default().with_selected(Some(0)),
            solve_list_state: ListState::default().with_selected(Some(0)),
            gen_algo_lookup: Vec::from([
                GenAlgorithms::Kruskal,
                GenAlgorithms::Prim,
                GenAlgorithms::Back,
                GenAlgorithms::AldousBroder,
                GenAlgorithms::GrowingTree(TreeSubAlgorithm::Newest),
                GenAlgorithms::HuntAndKill,
                GenAlgorithms::Wilson,
                GenAlgorithms::Eller,
                GenAlgorithms::Divide,
                GenAlgorithms::Sidewinder,
                GenAlgorithms::BinaryTree(BiasMethods::NorthWest),
            ]),
            solve_algo_lookup: Vec::from([
                SolveAlgorithms::Depth,
                SolveAlgorithms::Breadth,
                SolveAlgorithms::Dijkstra,
                SolveAlgorithms::AStar,
            ]),
            width: 2,
            height: 2,
            ratio: 0.5,
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

        self.maze_steps = str.split("\n\n").map(|s| s.to_string()).collect();
    }

    pub fn set_width(&mut self, size: usize) {
        self.width = size.clamp(2, self.max_size)
    }

    pub fn set_height(&mut self, size: usize) {
        self.height = size.clamp(2, self.max_size)
    }

    pub fn set_max_size(&mut self, size: usize) {
        self.max_size = size.max(2)
    }

    pub fn set_step_val(&mut self, step: usize) {
        self.step = step.clamp(0, self.maze_steps.len() - 1)
    }

    pub fn set_speed(&mut self, speed: usize) {
        self.speed = speed.clamp(1, 100)
    }

    pub fn set_ratio(&mut self, ratio: f64) {
        self.ratio = ratio.clamp(0.0, 1.0);
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

    pub fn get_ratio(&self) -> f64 {
        self.ratio
    }
}

impl GenAlgorithms {
    pub fn get_name(&self) -> String {
        match self {
            GenAlgorithms::Kruskal => "Kruskal".to_string(),
            GenAlgorithms::Prim => "Prim".to_string(),
            GenAlgorithms::Back => "Recursive Backtracking".to_string(),
            GenAlgorithms::AldousBroder => "Aldous-Broder".to_string(),
            GenAlgorithms::GrowingTree(method) => "Growing-Tree ".to_string() + &method.get_name(),
            GenAlgorithms::HuntAndKill => "Hunt-and-Kill".to_string(),
            GenAlgorithms::Wilson => "Wilson".to_string(),
            GenAlgorithms::Eller => "Eller".to_string(),
            GenAlgorithms::Divide => "Recursive Division".to_string(),
            GenAlgorithms::Sidewinder => "Sidewinder".to_string(),
            GenAlgorithms::BinaryTree(bias) => "Binary-Tree ".to_string() + &bias.to_string(),
        }
    }
}

impl TreeSubAlgorithm {
    pub fn get_name(&self) -> String {
        match self {
            TreeSubAlgorithm::Newest => "Newest".to_string(),
            TreeSubAlgorithm::Middle => "Middle".to_string(),
            TreeSubAlgorithm::Oldest => "Oldest".to_string(),
            TreeSubAlgorithm::Random => "Random".to_string(),
            TreeSubAlgorithm::NewestMiddle(ratio) => {
                format!("Newest-Middle {ratio:0.2}").to_string()
            }
            TreeSubAlgorithm::NewestOldest(ratio) => {
                format!("Newest-Oldest {ratio:0.2}").to_string()
            }
            TreeSubAlgorithm::NewestRandom(ratio) => {
                format!("Newest-Random {ratio:0.2}").to_string()
            }
            TreeSubAlgorithm::MiddleOldest(ratio) => {
                format!("Middle-Oldest {ratio:0.2}").to_string()
            }
            TreeSubAlgorithm::MiddleRandom(ratio) => {
                format!("Middle-Random {ratio:0.2}").to_string()
            }
            TreeSubAlgorithm::OldestRandom(ratio) => {
                format!("Oldest-Random {ratio:0.2}").to_string()
            }
        }
    }
}

impl SolveAlgorithms {
    pub fn get_name(&self) -> String {
        match self {
            SolveAlgorithms::Depth => "Depth First".to_string(),
            SolveAlgorithms::Breadth => "Breadth First".to_string(),
            SolveAlgorithms::Dijkstra => "Dijkstra".to_string(),
            SolveAlgorithms::AStar => "A-Star".to_string(),
        }
    }
}

impl ToString for GenAlgorithms {
    fn to_string(&self) -> String {
        match self {
            GenAlgorithms::Kruskal => "Kruskal".to_string(),
            GenAlgorithms::Prim => "Prim".to_string(),
            GenAlgorithms::Back => "Back".to_string(),
            GenAlgorithms::AldousBroder => "Aldous-Broder".to_string(),
            GenAlgorithms::GrowingTree(method) => "Growing-Tree ".to_string() + &method.to_string(),
            GenAlgorithms::HuntAndKill => "Hunt-and-Kill".to_string(),
            GenAlgorithms::Wilson => "Wilson".to_string(),
            GenAlgorithms::Eller => "Eller".to_string(),
            GenAlgorithms::Divide => "Divide".to_string(),
            GenAlgorithms::Sidewinder => "Sidewinder".to_string(),
            GenAlgorithms::BinaryTree(bias) => "Binary-Tree ".to_string() + &bias.to_string(),
        }
    }
}

impl ToString for TreeSubAlgorithm {
    fn to_string(&self) -> String {
        match self {
            TreeSubAlgorithm::Newest => "Newest".to_string(),
            TreeSubAlgorithm::Middle => "Middle".to_string(),
            TreeSubAlgorithm::Oldest => "Oldest".to_string(),
            TreeSubAlgorithm::Random => "Random".to_string(),
            TreeSubAlgorithm::NewestMiddle(ratio) => format!("Newest-Middle {ratio}").to_string(),
            TreeSubAlgorithm::NewestOldest(ratio) => format!("Newest-Oldest {ratio}").to_string(),
            TreeSubAlgorithm::NewestRandom(ratio) => format!("Newest-Random {ratio}").to_string(),
            TreeSubAlgorithm::MiddleOldest(ratio) => format!("Middle-Oldest {ratio}").to_string(),
            TreeSubAlgorithm::MiddleRandom(ratio) => format!("Middle-Random {ratio}").to_string(),
            TreeSubAlgorithm::OldestRandom(ratio) => {
                format!("Oldest-Random {ratio:0.2}").to_string()
            }
        }
    }
}

impl ToString for BiasMethods {
    fn to_string(&self) -> String {
        match self {
            BiasMethods::NorthWest => "NorthWest".to_string(),
            BiasMethods::NorthEast => "NorthEast".to_string(),
            BiasMethods::SouthWest => "SouthWest".to_string(),
            BiasMethods::SouthEast => "SouthEast".to_string(),
        }
    }
}

impl ToString for SolveAlgorithms {
    fn to_string(&self) -> String {
        match self {
            SolveAlgorithms::Depth => "Depth".to_string(),
            SolveAlgorithms::Breadth => "Breadth".to_string(),
            SolveAlgorithms::Dijkstra => "Dijkstra".to_string(),
            SolveAlgorithms::AStar => "A-Star".to_string(),
        }
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

        assert_eq!(
            expected, app.maze_steps,
            "Maze steps did not parse the maze correctly"
        );
        assert_eq!(
            3,
            app.maze_steps.len(),
            "Maze steps was not 3. Got {}",
            app.maze_steps.len()
        );
    }
}
