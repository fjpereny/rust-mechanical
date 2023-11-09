use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Rect, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let startup_msg = "Rust Mechanical Engineering Tools";
    let author_msg = "Frank Pereny (C) 2023";
    loop {
        terminal.draw(|frame| {
            frame.render_widget(Paragraph::new("").on_dark_gray(), frame.size());

            let half_width = frame.size().width / 2;
            let half_height = frame.size().height / 2;
            let quarter_height = frame.size().height / 4;

            let area = Rect {
                x: half_width - ((startup_msg.len() / 2) as u16),
                y: quarter_height,
                width: half_width,
                height: half_height,
            };
            frame.render_widget(
                Paragraph::new(startup_msg).light_blue().on_dark_gray(),
                area,
            );

            let area = Rect {
                x: half_width - ((author_msg.len() / 2) as u16),
                y: quarter_height + 1,
                width: half_width,
                height: half_height,
            };
            frame.render_widget(Paragraph::new(author_msg).light_blue().on_dark_gray(), area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('Q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
