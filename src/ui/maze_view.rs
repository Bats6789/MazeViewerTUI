use ratatui::{style::Color, widgets::Widget};

pub struct MazeView {
    cells: Option<Vec<Cell>>,
    height: usize,
    width: usize,
    default_color: Color,
    observed_color: Color,
    queued_color: Color,
    path_color: Color,
    route_color: Color,
}

struct Cell {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    queued: bool,
    observed: bool,
    path: bool,
    route: bool,
    start: bool,
    stop: bool,
    character: char,
}

impl Widget for MazeView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}

impl MazeView {
    pub fn load_maze(mut self, str: &str) {
        // Smallest maze possible is 29 characters
        if str.len() < 29 {
            return;
        }

        let rows: Vec<&str> = str.split('\n').filter(|&s| s != "").collect();

        self.height = (rows.len() - 1) / 2;
        self.width = (rows[0].len() - 1) / 2;

        let mut cells: Vec<Cell> = Vec::with_capacity(self.height * self.width);

        for i in 0..self.height * self.width {
            let y = 2 * (i / self.width) + 1;
            let x = 2 * (i % self.width) + 1;
            let up = rows[y - 1].chars().nth(x).unwrap() == '#';
            let down = rows[y + 1].chars().nth(x).unwrap() == '#';
            let left = rows[y].chars().nth(x - 1).unwrap() == '#';
            let right = rows[y].chars().nth(x + 1).unwrap() == '#';
            let character = rows[y].chars().nth(x).unwrap();
            let path = match character {
                '.' | '*' | 's' | 'x' | 'q' => true,
                _ => false,
            };
            let route = match character {
                '*' | 's' | 'x' | 'q' => true,
                _ => false,
            };
            let observed = character == ':';
            let queued = match character {
                'Q' | 'q' => true,
                _ => false,
            };
            let start = match character {
                'S' | 's' => true,
                _ => false,
            };
            let stop = match character {
                'X' | 'x' => true,
                _ => false,
            };

            let cell = Cell {
                up,
                down,
                left,
                right,
                path,
                route,
                observed,
                queued,
                start,
                stop,
                character,
            };

            cells[i] = cell;
        }

        self.cells = Some(cells);
    }

    pub fn new(self) -> MazeView {
        MazeView {
            cells: None,
            height: 0,
            width: 0,
            default_color: Color::White,
            observed_color: Color::LightRed,
            queued_color: Color::Red,
            path_color: Color::LightBlue,
            route_color: Color::from_u32(0x00FFD580),
        }
    }
}
