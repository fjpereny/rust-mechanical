pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

#[derive(Debug, Default)]
pub struct App {
    counter: i32,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1
    }

    pub fn decrement_counter(&mut self) {
        self.counter -= 1
    }

    pub fn get_count(&self) -> i32 {
        self.counter
    }

    pub fn get_should_quit(&self) -> bool {
        self.should_quit
    }
}
