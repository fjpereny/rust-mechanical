use crate::app::ui::popups::Popup;
use crate::app::ui::views::View;
use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

pub mod gas_details;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            if !app.command_line_active {
                match app.current_popup {
                    Popup::None => app.current_popup = Popup::QuitWarning,
                    _ => app.current_popup = Popup::None,
                }
            } else {
                app.command_line.clear();
                app.command_line_active = false;
            }
            app.quit_warning_popup.quit_button_selected = false;
        }
        KeyCode::Char(':') => match app.current_popup {
            Popup::None => {
                if !app.command_line_active {
                    app.command_line_active = true;
                }
            }
            _ => {}
        },
        KeyCode::Char('/') => match app.current_popup {
            Popup::None => {
                if !app.command_line_active {
                    app.command_line_active = true;
                }
            }
            _ => {}
        },
        _ => {}
    }

    if app.command_line_active {
        match key_event.code {
            KeyCode::Char(ch) => {
                if app.command_line_error {
                    app.command_line.text = String::from(":");
                    app.command_line_error = false;
                    if ch != ':' {
                        app.command_line.text.push(ch);
                    }
                } else {
                    app.command_line.text.push(ch);
                }
            }
            KeyCode::Enter => {
                let command = app.command_line.text.clone();
                let first_char = command.chars().nth(0);
                if first_char.is_some() && first_char.unwrap() == '/' {
                    // Search Here
                } else {
                    app.command(command);
                }
            }
            KeyCode::Backspace => {
                app.command_line.text.pop();
                if app.command_line.text == "".to_string() {
                    app.command_line_active = false
                }
            }
            _ => {}
        }
        return;
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
