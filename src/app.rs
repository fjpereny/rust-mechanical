pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

#[derive(Debug, Default)]
pub struct App {
    should_quit: bool,
    show_quit_menu: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            show_quit_menu: false,
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true
    }

    pub fn get_show_quit_menu(&self) -> bool {
        self.show_quit_menu
    }

    pub fn get_should_quit(&self) -> bool {
        self.should_quit
    }
}
