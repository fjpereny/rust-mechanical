use std::vec;

use super::popup::Popup;
use crate::app::App;
use ratatui::prelude::*;
use ratatui::style::*;
use ratatui::symbols;
use ratatui::widgets::*;

pub enum View {
    Main,
}

pub struct MainView {}

impl MainView {
    pub fn new() -> Self {
        MainView {}
    }

    pub fn show(app: &mut App, f: &mut Frame) {
        // let (color_fg, color_bg) = match app.current_popup {
        //     Popup::None => (Color::Cyan, Color::Black),
        //     _ => (Color::Gray, Color::Black),
        // };

        // let escape_msg = match app.current_popup {
        //     Popup::QuitWarning => "",
        //     _ => "Press `Esc` to quit.",
        // };
        // f.render_widget(
        //     Paragraph::new(escape_msg)
        //         .block(
        //             Block::default()
        //                 .title("Rust Mechanical Engineering Tools")
        //                 .title_alignment(Alignment::Left)
        //                 .borders(Borders::ALL)
        //                 .border_type(BorderType::Rounded),
        //         )
        //         .style(Style::default().fg(color_fg).bg(color_bg))
        //         .alignment(Alignment::Center),
        //     f.size(),
        // );
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(25)])
            .split(f.size());
        let sub_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        f.render_widget(Block::new().borders(Borders::ALL).title("Gas"), layout[0]);

        f.render_widget(
            Block::new().borders(Borders::ALL).title("Properties"),
            sub_layout[0],
        );

        f.render_widget(
            Block::new().borders(Borders::ALL).title("Calculation"),
            sub_layout[1],
        );

        let menu_items = &app.main_menu.items;
        let mut items: Vec<ListItem> = vec![];
        for i in menu_items {
            items.push(ListItem::new(*i));
        }

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Gas"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(items, layout[0], &mut app.main_menu.state);
    }
}
