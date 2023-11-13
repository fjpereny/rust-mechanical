pub mod app;
pub mod compression;
pub mod constants;
pub mod errors;
pub mod states;
pub mod units;

use crate::app::event::{Event, EventHandler};
use crate::app::tui::Tui;
use crate::app::update::update;
use crate::app::App;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::time::Instant;

const REFRESH_RATE_MILLISEC: u128 = 15;
const EVENT_POLL_RATE_MILLISEC: u64 = 100;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(EVENT_POLL_RATE_MILLISEC);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let mut last_draw_time = Instant::now();
    while !app.should_quit {
        let duration = Instant::now() - last_draw_time;
        if duration.as_millis() >= REFRESH_RATE_MILLISEC {
            tui.draw(&mut app)?;
            last_draw_time = Instant::now();
        }

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
