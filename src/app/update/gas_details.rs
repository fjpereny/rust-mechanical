use crate::app::ui::views::gas_detail::GasDetailWidget;
use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.modifiers {
        KeyModifiers::CONTROL => match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.gas_detail_active_menu = GasDetailWidget::TopRight
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.gas_detail_active_menu = GasDetailWidget::BottomRight
            }
            KeyCode::Left | KeyCode::Char('h') => {
                app.gas_detail_active_menu = GasDetailWidget::Left
            }
            KeyCode::Right | KeyCode::Char('l') => match app.gas_detail_active_menu {
                GasDetailWidget::Left => app.gas_detail_active_menu = GasDetailWidget::TopRight,
                _ => {}
            },
            _ => {}
        },
        _ => match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => match app.gas_detail_active_menu {
                GasDetailWidget::Left => app.gas_detail_menu.previous(),
                GasDetailWidget::TopRight => {}
                GasDetailWidget::BottomRight => {}
            },
            KeyCode::Down | KeyCode::Char('j') => match app.gas_detail_active_menu {
                GasDetailWidget::Left => app.gas_detail_menu.next(),
                GasDetailWidget::TopRight => {}
                GasDetailWidget::BottomRight => {}
            },
            _ => {}
        },
    }
}
