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

#[derive(Debug, PartialEq, Clone)]
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
        let str_width: u16 = u16::try_from(2 * self.width + 1).unwrap();
        let str_height: u16 = u16::try_from(2 * self.height + 1).unwrap();

        if str_width > area.width || str_height > area.height {
            panic!("Maze was larger than viewer");
        }

        if self.height == 0 || self.width == 0 {
            panic!("Maze wasn't allocated");
        }

        let x_gap = (area.width - str_width) / 2;
        let y_gap = (area.height - str_height) / 2;

        let start_x = area.x + x_gap;
        let start_y = area.y + y_gap;

        for (i, cell) in self.cells.as_ref().unwrap().iter().enumerate() {
            let x = 2 * u16::try_from(i % self.width).unwrap() + 1;
            let y = 2 * u16::try_from(i / self.width).unwrap() + 1;

            let (path_up, path_down, path_left, path_right) =
                get_paths(i, self.width, self.cells.as_ref().unwrap());
            let (route_up, route_down, route_left, route_right) =
                get_routes(i, self.width, self.cells.as_ref().unwrap());

            // upper wall
            if cell.up || path_up || route_up {
                buf.get_mut(start_x + x, start_y + y - 1)
                    .set_char(if cell.up { '━' } else { '─' })
                    .set_fg(if route_up {
                        self.route_color
                    } else if path_up {
                        self.path_color
                    } else {
                        self.default_color
                    });
            }

            // lower wall
            if cell.down || path_up || route_up {
                buf.get_mut(start_x + x, start_y + y + 1)
                    .set_char(if cell.down { '━' } else { '─' })
                    .set_fg(if route_down {
                        self.route_color
                    } else if path_down {
                        self.path_color
                    } else {
                        self.default_color
                    });
            }

            // left wall
            if cell.left || path_up || route_up {
                buf.get_mut(start_x + x - 1, start_y + y)
                    .set_char(if cell.left { '┃' } else { '│' })
                    .set_fg(if route_left {
                        self.route_color
                    } else if path_left {
                        self.path_color
                    } else {
                        self.default_color
                    });
            }

            // right wall
            if cell.right || path_up || route_up {
                buf.get_mut(start_x + x + 1, start_y + y)
                    .set_char(if cell.right { '┃' } else { '│' })
                    .set_fg(if route_right {
                        self.route_color
                    } else if path_right {
                        self.path_color
                    } else {
                        self.default_color
                    });
            }

            if cell.path || cell.route {
                buf.get_mut(start_x + x, start_y + y)
                    .set_char(get_path_symbol(cell.up, cell.down, cell.left, cell.right))
                    .set_fg(self.path_color);
            } else if cell.observed {
                buf.get_mut(start_x + x, start_y + y)
                    .set_char(' ')
                    .set_fg(self.observed_color);
            } else if cell.queued {
                buf.get_mut(start_x + x, start_y + y)
                    .set_char(' ')
                    .set_fg(self.queued_color);
            } else if cell.start || cell.stop {
                buf.get_mut(start_x + x, start_y + y)
                    .set_char(cell.character)
                    .set_fg(self.default_color);
            }


            // corners wall
            let (ulc, urc, llc, lrc) =
                get_corner_symbols(i, self.width, self.height, self.cells.as_ref().unwrap());

            buf.get_mut(start_x + x - 1, start_y + y - 1)
                .set_char(ulc)
                .set_fg(self.default_color);

            buf.get_mut(start_x + x + 1, start_y + y - 1)
                .set_char(urc)
                .set_fg(self.default_color);

            buf.get_mut(start_x + x - 1, start_y + y + 1)
                .set_char(llc)
                .set_fg(self.default_color);

            buf.get_mut(start_x + x + 1, start_y + y + 1)
                .set_char(lrc)
                .set_fg(self.default_color);
        }
    }
}

impl MazeView {
    pub fn load_maze(&mut self, str: &str) {
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

            cells.push(cell);
        }

        self.cells = Some(cells);
    }

    pub fn new() -> MazeView {
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

fn get_corner_symbols(
    index: usize,
    width: usize,
    height: usize,
    cells: &Vec<Cell>,
) -> (char, char, char, char) {
    let x = index % width;
    let y = index / width;

    let up = y > 0 && cells[index - width].left;
    let down = cells[index].left;
    let left = x > 0 && cells[index - 1].up;
    let right = cells[index].up;

    let ulc = get_wall_symbol(up, down, left, right);

    let up = y > 0 && cells[index - width].right;
    let down = cells[index].right;
    let left = cells[index].up;
    let right = x < width - 1 && cells[index + 1].up;

    let urc = get_wall_symbol(up, down, left, right);

    let up = cells[index].left;
    let down = y < height - 1 && cells[index + width].left;
    let left = x > 0 && cells[index - 1].down;
    let right = cells[index].down;

    let llc = get_wall_symbol(up, down, left, right);

    let up = cells[index].right;
    let down = y < height - 1 && cells[index + width].right;
    let left = cells[index].down;
    let right = x < width - 1 && cells[index + 1].down;

    let lrc = get_wall_symbol(up, down, left, right);

    (ulc, urc, llc, lrc)
}

fn get_wall_symbol(up: bool, down: bool, left: bool, right: bool) -> char {
    if up && down && left && right {
        '╋'
    } else if !up && down && left && right {
        '┳'
    } else if up && !down && left && right {
        '┻'
    } else if !up && !down && left && right {
        '━'
    } else if up && down && !left && right {
        '┣'
    } else if !up && down && !left && right {
        '┏'
    } else if up && !down && !left && right {
        '┗'
    } else if !up && !down && !left && right {
        '╺'
    } else if up && down && left && !right {
        '┫'
    } else if !up && down && left && !right {
        '┓'
    } else if up && !down && left && !right {
        '┛'
    } else if !up && !down && left && !right {
        '╸'
    } else if up && down && !left && !right {
        '┃'
    } else if !up && down && !left && !right {
        '╻'
    } else if up && !down && !left && !right {
        '╹'
    } else {
        ' '
    }
}

fn get_paths(index: usize, width: usize, cells: &Vec<Cell>) -> (bool, bool, bool, bool) {
    let up = !cells[index].up && cells[index].path && cells[index - width].path;
    let down = !cells[index].down && cells[index].path && cells[index + width].path;
    let left = !cells[index].left && cells[index].path && cells[index - 1].path;
    let right = !cells[index].right && cells[index].path && cells[index + 1].path;

    (up, down, left, right)
}

fn get_routes(index: usize, width: usize, cells: &Vec<Cell>) -> (bool, bool, bool, bool) {
    let up = !cells[index].up && cells[index].route && cells[index - width].route;
    let down = !cells[index].down && cells[index].route && cells[index + width].route;
    let left = !cells[index].left && cells[index].route && cells[index - 1].route;
    let right = !cells[index].right && cells[index].route && cells[index + 1].route;

    (up, down, left, right)
}

fn get_path_symbol(up: bool, down: bool, left: bool, right: bool) -> char {
    if up && down && left && right {
        '┼'
    } else if !up && down && left && right {
        '┬'
    } else if up && !down && left && right {
        '┴'
    } else if !up && !down && left && right {
        '─'
    } else if up && down && !left && right {
        '├'
    } else if !up && down && !left && right {
        '┌'
    } else if up && !down && !left && right {
        '└'
    } else if !up && !down && !left && right {
        '╶'
    } else if up && down && left && !right {
        '┤'
    } else if !up && down && left && !right {
        '┐'
    } else if up && !down && left && !right {
        '┘'
    } else if !up && !down && left && !right {
        '╴'
    } else if up && down && !left && !right {
        '│'
    } else if !up && down && !left && !right {
        '╷'
    } else if up && !down && !left && !right {
        '╵'
    } else {
        ' '
    }
}

#[cfg(test)]
mod maze_view_tests {
    use std::iter::zip;

    use super::*;

    #[test]
    fn create_maze() {
        let mut maze_view = MazeView::new();

        maze_view.load_maze(
            "\
#####
# # #
#####
# # #
#####",
        );

        println!("Made it");
        assert_eq!(
            maze_view.height, 2,
            "Expected height to be 2, got {}",
            maze_view.height
        );
        assert_eq!(
            maze_view.width, 2,
            "Expected height to be 2, got {}",
            maze_view.width
        );

        for cell in maze_view.cells.unwrap() {
            assert_eq!(cell.up, true, "cell.up was false");
            assert_eq!(cell.down, true, "cell.down was false");
            assert_eq!(cell.left, true, "cell.left was false");
            assert_eq!(cell.right, true, "cell.right was false");
            assert_eq!(
                cell.character, ' ',
                "cell.character was '{}'",
                cell.character
            );
            assert_eq!(cell.path, false, "cell.path was true");
            assert_eq!(cell.route, false, "cell.route was true");
            assert_eq!(cell.observed, false, "cell.observed was true");
            assert_eq!(cell.queued, false, "cell.queued was true");
            assert_eq!(cell.start, false, "cell.start was true");
            assert_eq!(cell.stop, false, "cell.stop was true");
        }
    }

    #[test]
    fn symbol_detection() {
        let mut maze_view = MazeView::new();

        maze_view.load_maze(
            "\
#######
#Q : x#
# #####
#. *  #
##### #
#s    #
#######",
        );

        assert_eq!(maze_view.height, 3);
        assert_eq!(maze_view.width, 3);

        let mut expected: Vec<Cell> = Vec::with_capacity(maze_view.height * maze_view.width);

        for _ in 0..(maze_view.height * maze_view.width) {
            expected.push(Cell {
                up: true,
                down: true,
                left: true,
                right: true,
                character: ' ',
                path: false,
                route: false,
                observed: false,
                queued: false,
                start: false,
                stop: false,
            });
        }

        expected[0].character = 'Q';
        expected[0].right = false;
        expected[0].down = false;
        expected[0].queued = true;

        expected[1].character = ':';
        expected[1].right = false;
        expected[1].left = false;
        expected[1].observed = true;

        expected[2].character = 'x';
        expected[2].left = false;
        expected[2].path = true;
        expected[2].route = true;
        expected[2].stop = true;

        expected[3].character = '.';
        expected[3].up = false;
        expected[3].right = false;
        expected[3].path = true;

        expected[4].character = '*';
        expected[4].left = false;
        expected[4].right = false;
        expected[4].path = true;
        expected[4].route = true;

        expected[5].left = false;
        expected[5].down = false;

        expected[6].character = 's';
        expected[6].right = false;
        expected[6].path = true;
        expected[6].route = true;
        expected[6].start = true;

        expected[7].left = false;
        expected[7].right = false;

        expected[8].left = false;
        expected[8].up = false;

        for (i, (left, right)) in zip(maze_view.cells.unwrap(), expected).enumerate() {
            assert_eq!(left, right, "{}: {:?} != {:?}", i, left, right);
        }
    }
}
