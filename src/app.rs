use ratatui::widgets::ListState;
use rust_mechanical::constants::gas::*;
use ui::command_line::CommandLine;
use ui::popups::quit_warning_popup::QuitWarningPopup;
use ui::themes::Theme;
use ui::views::gas_detail::GasDetailWidget;
use ui::{popups::Popup, views::View};

pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

pub struct App {
    pub current_theme: Theme,
    pub should_quit: bool,

    pub current_view: View,
    pub current_popup: Popup,

    pub command_line_active: bool,
    command_line: CommandLine,
    command_line_error: bool,

    gas_detail_menu: StatefulList<Gas>,
    gas_detail_active_menu: GasDetailWidget,

    pub quit_warning_popup: QuitWarningPopup,
}

impl App {
    pub fn new() -> Self {
        App {
            current_theme: Theme::Default,
            should_quit: false,

            current_view: View::GasDetailView,
            current_popup: Popup::None,

            command_line_active: false,
            command_line: CommandLine::new(),
            command_line_error: false,

            gas_detail_menu: StatefulList::with_items(gas_list()),
            gas_detail_active_menu: GasDetailWidget::Left,

            quit_warning_popup: QuitWarningPopup::new(),
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true
    }

    pub fn command(&mut self, command_str: String) {
        match command_str.as_str() {
            ":q!" => self.should_quit = true,
            _ => {
                self.command_line_error = true;
                self.command_line.text = String::from("Unknown command!");
            }
        }
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
