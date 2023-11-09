use crate::app::App;
use ratatui::prelude::{Alignment, Frame};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use super::popup::Popup;

pub enum View {
    Main,
}

pub struct MainView {}

impl MainView {
    pub fn new() -> Self {
        MainView {}
    }

    pub fn show(app: &App, f: &mut Frame) {
        let (color_fg, color_bg) = match app.current_popup {
            Popup::None => (Color::Cyan, Color::Black),
            _ => (Color::Gray, Color::Black),
        };

        let escape_msg = match app.current_popup {
            Popup::QuitWarning => "",
            _ => "Press `Esc` to quit.",
        };
        f.render_widget(
            Paragraph::new(escape_msg)
                .block(
                    Block::default()
                        .title("Rust Mechanical Engineering Tools")
                        .title_alignment(Alignment::Left)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(color_fg).bg(color_bg))
                .alignment(Alignment::Center),
            f.size(),
        );
    }
}
