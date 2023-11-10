use ratatui::widgets::ListState;
use rust_mechanical::constants::gas::*;
use ui::popups::quit_warning_popup::QuitWarningPopup;
use ui::views::gas_detail::GasDetailWidget;
use ui::{popups::Popup, views::View};

pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

pub struct App {
    pub should_quit: bool,

    pub current_view: View,
    pub current_popup: Popup,

    gas_detail_menu: StatefulList<Gas>,
    gas_detail_active_menu: GasDetailWidget,

    pub quit_warning_popup: QuitWarningPopup,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,

            current_view: View::GasDetailView,
            current_popup: Popup::None,

            gas_detail_menu: StatefulList::with_items(gas_list()),
            gas_detail_active_menu: GasDetailWidget::Left,

            quit_warning_popup: QuitWarningPopup::new(),
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
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
