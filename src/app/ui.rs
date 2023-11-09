pub mod popup;
pub mod view;

use popup::Popup;

use ratatui::{
    layout::Layout,
    prelude::{Alignment, Constraint, Direction, Frame, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
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
        Popup::CloseWarning => {
            let area = centered_rect(33, 33, f.size());
            let button_area = centered_rect(50, 30, area);
            let popup = Paragraph::new(
                "Are you sure you want to quit?\n\
            Unsaved changes will be lost!",
            )
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Quit Rust Mechanical")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::Red)),
            );

            let quit_button_fg_color;
            let quit_button_border_type;
            match app.quit_warning_popup.quit_button_selected {
                true => {
                    quit_button_fg_color = Color::White;
                    quit_button_border_type = BorderType::Double;
                }
                false => {
                    quit_button_fg_color = Color::Red;
                    quit_button_border_type = BorderType::Rounded;
                }
            }

            let quit_button = Paragraph::new("Quit").alignment(Alignment::Center).block(
                Block::default()
                    .borders(Borders::ALL)
                    .padding(Padding::zero())
                    .border_type(quit_button_border_type)
                    .style(Style::default().fg(quit_button_fg_color)),
            );

            f.render_widget(Clear, area);
            f.render_widget(popup, area);
            f.render_widget(quit_button, button_area);
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
