pub mod popup;
pub mod view;

use popup::Popup;
use view::View;

use ratatui::{
    layout::Layout,
    prelude::{Constraint, Direction, Frame, Rect},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    match app.current_view {
        View::Main => view::MainView::show(app, f),
    }

    match app.current_popup {
        Popup::None => {}
        Popup::QuitWarning => popup::QuitWarningPopup::show(app, f),
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
