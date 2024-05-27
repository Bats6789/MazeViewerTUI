pub enum CurrentScreen {
    Main, Size
}

pub enum SizeSetting {
    Width, Height
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub size_setting: SizeSetting,
    pub size: usize,
    width: usize,
    height: usize,
    max_size: usize
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            size_setting: SizeSetting::Width,
            size: 2,
            width: 2,
            height: 2,
            max_size: 2
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
        } else if size > self.max_size {
            self.max_size = self.max_size;
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
