use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            app.quit_button_active = false;
            app.show_quit_menu = !app.show_quit_menu
        }
        _ => {}
    }

    if app.show_quit_menu {
        match key_event.code {
            KeyCode::Left | KeyCode::Right => app.quit_button_active = !app.quit_button_active,
            KeyCode::Enter | KeyCode::Char(' ') => match app.quit_button_active {
                true => app.should_quit = true,
                false => {}
            },
            _ => {}
        }
    }
}
