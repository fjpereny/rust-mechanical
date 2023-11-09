use self::ui::popup::QuitWarningPopup;
use self::ui::{popup::Popup, view::View};

pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

pub struct App {
    pub should_quit: bool,
    pub current_view: View,
    pub current_popup: Popup,

    pub quit_warning_popup: QuitWarningPopup,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            current_view: View::Main,
            current_popup: Popup::None,

            quit_warning_popup: QuitWarningPopup::new(),
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true
    }
}
