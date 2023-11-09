use crate::app::ui::popup::Popup;
use crate::app::ui::view::View;
use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            match app.current_popup {
                Popup::None => app.current_popup = Popup::QuitWarning,
                _ => app.current_popup = Popup::None,
            }
            app.quit_warning_popup.quit_button_selected = false;
        }
        _ => {}
    }

    match app.current_view {
        View::Main => match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => app.main_menu.previous(),
            KeyCode::Down | KeyCode::Char('j') => app.main_menu.next(),
            _ => {}
        },
    }

    match app.current_popup {
        Popup::None => {}
        Popup::QuitWarning => match key_event.code {
            KeyCode::Left | KeyCode::Right => {
                app.quit_warning_popup.quit_button_selected =
                    !app.quit_warning_popup.quit_button_selected
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                match app.quit_warning_popup.quit_button_selected {
                    true => app.should_quit = true,
                    false => {
                        app.current_popup = Popup::None;
                        app.quit_warning_popup.quit_button_selected = false;
                    }
                }
            }
            _ => {}
        },
    }
}
