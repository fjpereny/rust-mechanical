pub mod gas_details;

use crate::app::ui::popups::Popup;
use crate::app::ui::views::View;
use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

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

    match app.current_view {
        View::GasDetailView => gas_details::update(app, key_event),
    }
}
