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

        let menu_items = &app.main_menu.items;
        let mut items: Vec<ListItem> = vec![];
        for item in menu_items {
            items.push(ListItem::new(item.name));
        }
        let list1 = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Gas"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(list1, layout[0], &mut app.main_menu.state);

        let sub_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        f.render_widget(
            Block::new().borders(Borders::ALL).title("Properties"),
            sub_layout[0],
        );

        f.render_widget(
            Block::new().borders(Borders::ALL).title("Calculation"),
            sub_layout[1],
        );

        let selected_index = app.main_menu.state.selected();
        match selected_index {
            Some(index) => {
                let selected_gas = app.main_menu.items.get(index);
                match selected_gas {
                    Some(g) => {
                        if selected_gas.is_some() {
                            let s: String;
                            if g.chemical_formula.is_empty() {
                                s = format!("{}", g.name());
                            } else {
                                s = format!("{} ({})", g.name(), g.chemical_formula);
                            }
                            let p = Paragraph::new(s);
                            let mut layout = sub_layout[0].clone();
                            layout.x += 3;
                            layout.y += 2;
                            layout.width -= 3;
                            layout.height = 1;
                            f.render_widget(p, layout);
                        }

                        let s = format!("Density: {} kg/m^3", g.standard_density());
                        let p = Paragraph::new(s);
                        let mut layout = sub_layout[0].clone();
                        layout.x += 3;
                        layout.y += 3;
                        layout.width -= 3;
                        layout.height = 1;
                        f.render_widget(p, layout);

                        let s = format!("Specific Heat Ratio (cp/cv): {}", g.specific_heat_ratio());
                        let p = Paragraph::new(s);
                        let mut layout = sub_layout[0].clone();
                        layout.x += 3;
                        layout.y += 4;
                        layout.width -= 3;
                        layout.height = 1;
                        f.render_widget(p, layout);
                    }
                    None => (),
                }
            }
            None => {}
        }
    }
}
