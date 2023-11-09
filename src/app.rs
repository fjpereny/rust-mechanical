use self::ui::popup::QuitWarningPopup;
use self::ui::{popup::Popup, views::View};
use ratatui::widgets::ListState;
use rust_mechanical::constants::gas::*;

pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

pub struct App {
    pub should_quit: bool,

    pub current_view: View,
    pub current_popup: Popup,

    main_menu: StatefulList<Gas>,

    pub quit_warning_popup: QuitWarningPopup,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,

            current_view: View::Main,
            current_popup: Popup::None,

            main_menu: StatefulList::with_items(gas_list()),

            quit_warning_popup: QuitWarningPopup::new(),
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true
    }
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut list = StatefulList {
            state: ListState::default(),
            items,
        };
        let first_item = list.items.first();
        match first_item {
            Some(_) => list.state.select(Some(0)),
            None => {}
        }
        list
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}
