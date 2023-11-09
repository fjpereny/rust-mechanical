use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::vec;

pub struct GasDetailView {}

impl GasDetailView {
    pub fn new() -> Self {
        GasDetailView {}
    }

    pub fn show(app: &mut App, f: &mut Frame) {
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
        for item in menu_items {
            items.push(ListItem::new(item.name));
        }

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Gas"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_stateful_widget(list, layout[0], &mut app.main_menu.state);
    }
}
