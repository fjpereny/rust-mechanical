use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!(
            "Press `Esc` to quit.\n\
        Counter: {}",
            app.counter
        ))
        .block(
            Block::default()
                .title("Rust Mechanical Engineering Tools")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::LightBlue))
        .alignment(Alignment::Center),
        f.size(),
    )
}
