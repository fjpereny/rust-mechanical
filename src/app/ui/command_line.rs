use crate::app::App;
use ratatui::prelude::{Alignment, Frame, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

pub struct CommandLine {
    pub text: String,
}

impl CommandLine {
    pub fn new() -> Self {
        CommandLine {
            text: String::from(""),
        }
    }

    pub fn clear(&mut self) {
        self.text = String::from("")
    }

    pub fn show(app: &App, f: &mut Frame) {
        let area = Rect::new(0, f.size().height - 1, f.size().width, 1);
        let popup = Paragraph::new(format!("{}", app.command_line.text))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .style(Style::default().fg(Color::LightGreen)),
            );

        f.render_widget(Clear, area);
        f.render_widget(popup, area);
    }
}

impl Default for CommandLine {
    fn default() -> Self {
        CommandLine::new()
    }
}
