use crate::app::App;
use popups::quit_warning_popup::QuitWarningPopup;
use popups::Popup;
use ratatui::{
    layout::Layout,
    prelude::{Constraint, Direction, Frame, Rect},
};
use views::gas_detail::GasDetailView;
use views::View;

pub mod popups;
pub mod views;

pub fn render(app: &mut App, f: &mut Frame) {
    match app.current_view {
        View::GasDetailView => GasDetailView::show(app, f),
    }

    match app.current_popup {
        Popup::None => {}
        Popup::QuitWarning => QuitWarningPopup::show(app, f),
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
