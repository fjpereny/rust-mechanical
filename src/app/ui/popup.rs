use crate::app::ui::centered_rect;
use crate::app::App;
use ratatui::prelude::{Alignment, Frame};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph};

pub enum Popup {
    None,
    QuitWarning,
}

pub struct QuitWarningPopup {
    pub quit_button_selected: bool,
}

impl QuitWarningPopup {
    pub fn new() -> Self {
        QuitWarningPopup {
            quit_button_selected: false,
        }
    }

    pub fn show(app: &App, f: &mut Frame) {
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
