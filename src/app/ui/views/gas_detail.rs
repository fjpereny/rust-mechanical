use crate::app::ui::themes::palettes::*;
use crate::app::ui::themes::Theme;
use crate::app::App;
use mechanical_engineering::constants::gas_properties::air;
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::vec;

pub enum GasDetailWidget {
    Left,
    TopRight,
    BottomRight,
}

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

        let menu_items = &app.gas_detail_menu.items;
        let mut items: Vec<ListItem> = vec![];
        for item in menu_items {
            items.push(ListItem::new(item.name));
        }

        let border_type = BorderType::Rounded;
        let active_border_style = Style::default().cyan().on_dark_gray();
        let inactive_border_style = Style::default().gray().on_dark_gray();

        let left_block = match app.gas_detail_active_menu {
            GasDetailWidget::Left => Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
                .border_style(active_border_style)
                .title("Gas"),
            _ => Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
                .border_style(inactive_border_style)
                .title("Gas"),
        };

        let top_right_block = match app.gas_detail_active_menu {
            GasDetailWidget::TopRight => Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
                .border_style(active_border_style)
                .title("Properties"),
            _ => Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
                .border_style(inactive_border_style)
                .title("Properties"),
        };

        let bottom_right_block = match app.gas_detail_active_menu {
            GasDetailWidget::BottomRight => Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
                .border_style(active_border_style)
                .title("Calculation"),
            _ => Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
                .border_style(inactive_border_style)
                .title("Calculation"),
        };

        let list1 = List::new(items)
            .block(left_block)
            .highlight_style(
                Style::default()
                    .bg(Color::LightBlue)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(list1, layout[0], &mut app.gas_detail_menu.state);

        let sub_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        f.render_widget(top_right_block, sub_layout[0]);

        f.render_widget(bottom_right_block, sub_layout[1]);

        let selected_index = app.gas_detail_menu.state.selected();
        match selected_index {
            Some(index) => {
                let selected_gas = app.gas_detail_menu.items.get(index);
                match selected_gas {
                    Some(g) => {
                        if selected_gas.is_some() {
                            let mut y_offset = 1;
                            let x_offset = 1;

                            let row_color_fg_1;
                            let row_color_bg_1;
                            let row_color_fg_2;
                            let row_color_bg_2;
                            match app.current_theme {
                                Theme::Default => {
                                    row_color_fg_1 = DEFAULT_PALETTE.row_color_fg_a;
                                    row_color_bg_1 = DEFAULT_PALETTE.row_color_bg_a;
                                    row_color_fg_2 = DEFAULT_PALETTE.row_color_fg_b;
                                    row_color_bg_2 = DEFAULT_PALETTE.row_color_bg_b;
                                }
                            }

                            let s: String;
                            if g.chemical_formula.is_empty() {
                                s = format!("{}", g.name());
                            } else {
                                s = format!("{} ({})", g.name(), g.chemical_formula);
                            }
                            let mut p = Paragraph::new(s);
                            p = p.set_style(Style::default().fg(Color::LightGreen).bold());
                            let mut layout = sub_layout[0];
                            layout.x += x_offset;
                            layout.y += y_offset;
                            layout.width -= 3;
                            layout.height = 1;
                            f.render_widget(p, layout);

                            y_offset += 1;
                            let mut row_color_toggle = true;
                            let s = format!("Density: {:.4} kg/m^3", g.standard_density());
                            let mut p = Paragraph::new(s);
                            if row_color_toggle {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_1).bg(row_color_bg_1),
                                );
                            } else {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_2).bg(row_color_bg_2),
                                );
                            }
                            let mut layout = sub_layout[0];
                            layout.x += x_offset;
                            layout.y += y_offset;
                            layout.width -= 3;
                            layout.height = 1;
                            f.render_widget(p, layout);

                            y_offset += 1;
                            row_color_toggle = !row_color_toggle;
                            let s =
                                format!("Specific Heat Cp: {:.4} kJ/kg-K", g.specific_heat_cp());
                            let mut p = Paragraph::new(s);
                            if row_color_toggle {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_1).bg(row_color_bg_1),
                                );
                            } else {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_2).bg(row_color_bg_2),
                                );
                            }
                            let mut layout = sub_layout[0];
                            layout.x += x_offset;
                            layout.y += y_offset;
                            layout.width -= 3;
                            layout.height = 1;
                            f.render_widget(p, layout);

                            y_offset += 1;
                            row_color_toggle = !row_color_toggle;
                            let s = format!("Specific Heat Cv:");
                            let mut p = Paragraph::new(s);
                            if row_color_toggle {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_1).bg(row_color_bg_1),
                                );
                            } else {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_2).bg(row_color_bg_2),
                                );
                            }
                            let mut layout = sub_layout[0];
                            layout.x += x_offset;
                            layout.y += y_offset;
                            layout.width -= 3;
                            layout.height = 1;
                            f.render_widget(p, layout);

                            let s = format!("{:.4}", g.specific_heat_cv());
                            let mut p = Paragraph::new(s);
                            if row_color_toggle {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_1).bg(row_color_bg_1),
                                );
                            } else {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_2).bg(row_color_bg_2),
                                );
                            }
                            let mut layout = sub_layout[0];
                            layout.x += x_offset + 29;
                            layout.y += y_offset;
                            layout.width -= 30;
                            layout.height = 1;
                            f.render_widget(p, layout);

                            y_offset += 1;
                            row_color_toggle = !row_color_toggle;
                            let s = format!(
                                "Specific Heat Ratio (Cp/Cv): {:.4}",
                                g.specific_heat_ratio()
                            );
                            let mut p = Paragraph::new(s);
                            if row_color_toggle {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_1).bg(row_color_bg_1),
                                );
                            } else {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_2).bg(row_color_bg_2),
                                );
                            }
                            let mut layout = sub_layout[0];
                            layout.x += x_offset;
                            layout.y += y_offset;
                            layout.width -= 3;
                            layout.height = 1;
                            f.render_widget(p, layout);

                            y_offset += 1;
                            row_color_toggle = !row_color_toggle;
                            let s = format!("Molar Mass: {:.4} g/mol", g.molar_mass());
                            let mut p = Paragraph::new(s);
                            if row_color_toggle {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_1).bg(row_color_bg_1),
                                );
                            } else {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_2).bg(row_color_bg_2),
                                );
                            }
                            let mut layout = sub_layout[0];
                            layout.x += x_offset;
                            layout.y += y_offset;
                            layout.width -= 3;
                            layout.height = 1;
                            f.render_widget(p, layout);

                            y_offset += 1;
                            row_color_toggle = !row_color_toggle;
                            let s = format!(
                                "Calulated (cp/cv): {:.4}",
                                air::specific_heat_ratio::interpolate(225.0, 0.05).unwrap()
                            );
                            let mut p = Paragraph::new(s);
                            if row_color_toggle {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_1).bg(row_color_bg_1),
                                );
                            } else {
                                p = p.set_style(
                                    Style::default().fg(row_color_fg_2).bg(row_color_bg_2),
                                );
                            }
                            let mut layout = sub_layout[0];
                            layout.x += x_offset;
                            layout.y += y_offset;
                            layout.width -= 3;
                            layout.height = 1;
                            f.render_widget(p, layout);
                        }
                    }
                    None => (),
                }
            }
            None => {}
        }
    }
}

impl Default for GasDetailView {
    fn default() -> Self {
        GasDetailView::new()
    }
}
