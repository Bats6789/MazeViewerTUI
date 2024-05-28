use ratatui::style::Color;

pub enum CurrentScreen {
    Main, Size
}

#[derive(PartialEq)]
pub enum SizeSetting {
    Width, Height
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub size_setting: SizeSetting,
    pub size: usize,
    pub default_color: Color,
    pub highlight_fg: Color,
    pub highlight_bg: Color,
    width: usize,
    height: usize,
    max_size: usize,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            size_setting: SizeSetting::Width,
            size: 2,
            width: 2,
            height: 2,
            max_size: 2,
            default_color: Color::White,
            highlight_fg: Color::Black,
            highlight_bg: Color::Yellow
        }
    }

    pub fn set_width(&mut self, size: usize) {
        if size < 2 {
            self.width = 2;
        } else if size > self.max_size {
            self.width = self.max_size;
        } else {
            self.width = size;
        }
    }

    pub fn set_height(&mut self, size: usize) {
        if size < 2 {
            self.height = 2;
        } else if size > self.max_size {
            self.height = self.max_size;
        } else {
            self.height = size;
        }
    }

    pub fn set_max_size(&mut self, size: usize) {
        if size < 2 {
            self.max_size = 2;
        } else {
            self.max_size = size;
        }
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
}

#[cfg(test)]
mod app_tests {
    use super::*;

    #[test]
    fn set_height_test() {
        let mut app = App::new();

        app.set_max_size(10);

        assert_eq!(app.get_height(), 2, "App.height should start as 2. Got {} instead", app.get_height());

        app.set_height(5);

        assert_eq!(app.get_height(), 5, "App.height should have changed to 5. Got {} instead", app.get_height());

        app.set_height(11);

        assert_eq!(app.get_height(), 10, "App.height should not go above 10. Got {} instead", app.get_height());
    }

    #[test]
    fn set_width_test() {
        let mut app = App::new();

        app.set_max_size(10);

        assert_eq!(app.get_width(), 2, "App.width should start as 2. Got {} instead", app.get_width());

        app.set_width(5);

        assert_eq!(app.get_width(), 5, "App.width should have changed to 5. Got {} instead", app.get_width());

        app.set_width(11);

        assert_eq!(app.get_width(), 10, "App.width should not go above 10. Got {} instead", app.get_width());
    }
}
