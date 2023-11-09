pub mod popup;
pub mod view;

use popup::{Popup, QuitWarningPopup};

use ratatui::{
    layout::Layout,
    prelude::{Alignment, Constraint, Direction, Frame, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new("Press `Esc` to quit.")
            .block(
                Block::default()
                    .title("Rust Mechanical Engineering Tools")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::LightBlue))
            .alignment(Alignment::Center),
        f.size(),
    );

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
