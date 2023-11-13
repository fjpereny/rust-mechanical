use crate::app::ui::themes::palettes::*;
use crate::app::ui::themes::Theme;
use crate::app::App;
use crate::constants::gas_properties::air;
use crate::units::pressure;
use crate::units::temperature;
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::vec;

pub enum GasDetailWidget {
    Left,
    TopRight,
    BottomRight,
}

pub struct GasDetailViewState {
    pressure: pressure::Pressure,
    temperature: temperature::Temperature,

    // Calculated
    specific_heat_ratio: f32,
}

impl GasDetailViewState {
    pub fn new() -> Self {
        let pressure = pressure::Pressure::new(1.0, pressure::Unit::Atm, true);
        let temperature = temperature::Temperature::new(273.15, temperature::Unit::K);
        let specific_heat_ratio =
            air::specific_heat_ratio::interpolate(temperature.value(), pressure.value())
                .expect("Error: specific heat ratio interplation failed!");

        Self {
            pressure,
            temperature,

            specific_heat_ratio,
        }
    }

    pub fn update(&mut self) {
        let mut pressure_atm = self.pressure.clone();
        pressure_atm.convert_unit(pressure::Unit::Atm);

        let mut temperature_k = self.temperature.clone();
        temperature_k.convert_unit(temperature::Unit::K);

        self.specific_heat_ratio = match air::specific_heat_ratio::interpolate(
            temperature_k.value(),
            pressure_atm.value(),
        ) {
            Some(val) => val,
            _ => -1.0,
        };
    }

    pub fn set_pressure_state(&mut self, pressure: pressure::Pressure) {
        self.pressure = pressure;
        self.update();
    }

    pub fn set_temperature_state(&mut self, temperature: temperature::Temperature) {
        self.temperature = temperature;
        self.update();
    }

    pub fn set_units_si(&mut self) {
        self.pressure.convert_unit(pressure::Unit::Kpa);
        self.temperature.convert_unit(temperature::Unit::C);
        self.update();
    }

    pub fn set_units_us(&mut self) {
        self.pressure.convert_unit(pressure::Unit::Psi);
        self.temperature.convert_unit(temperature::Unit::F);
        self.update();
    }
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
                            let header_color_1;
                            let row_color_fg_1;
                            let row_color_bg_1;
                            let row_color_fg_2;
                            let row_color_bg_2;
                            match app.current_theme {
                                Theme::Default => {
                                    header_color_1 = DEFAULT_PALETTE.header_color_1;
                                    row_color_fg_1 = DEFAULT_PALETTE.row_color_fg_a;
                                    row_color_bg_1 = DEFAULT_PALETTE.row_color_bg_a;
                                    row_color_fg_2 = DEFAULT_PALETTE.row_color_fg_b;
                                    row_color_bg_2 = DEFAULT_PALETTE.row_color_bg_b;
                                }
                            }

                            let x_offset = 1;
                            let mut y_offset = 0;
                            let mut row_color_toggle = true;

                            render_row(
                                format!("{}", g.name()),
                                sub_layout[0],
                                x_offset,
                                &mut y_offset,
                                header_color_1,
                                row_color_bg_1,
                                header_color_1,
                                row_color_bg_2,
                                &mut row_color_toggle,
                                f,
                            );

                            render_row(
                                format!(
                                    "Pressure: {:.4} {}",
                                    app.gas_detail_view_state.pressure.value(),
                                    app.gas_detail_view_state.pressure.unit().to_string(),
                                ),
                                sub_layout[0],
                                x_offset,
                                &mut y_offset,
                                row_color_fg_1,
                                row_color_bg_1,
                                row_color_fg_2,
                                row_color_bg_2,
                                &mut row_color_toggle,
                                f,
                            );

                            render_row(
                                format!(
                                    "Temperature: {:.4} {}",
                                    app.gas_detail_view_state.temperature.value(),
                                    app.gas_detail_view_state.temperature.unit().to_string(),
                                ),
                                sub_layout[0],
                                x_offset,
                                &mut y_offset,
                                row_color_fg_1,
                                row_color_bg_1,
                                row_color_fg_2,
                                row_color_bg_2,
                                &mut row_color_toggle,
                                f,
                            );

                            let specific_heat_ratio = app.gas_detail_view_state.specific_heat_ratio;
                            let specific_heat_ratio = match specific_heat_ratio < 0.0 {
                                true => "N/A".to_string(),
                                _ => format!("{specific_heat_ratio:.4}"),
                            };
                            render_row(
                                format!("Specific Heat Ratio (Cp/Cv): {specific_heat_ratio}",),
                                sub_layout[0],
                                x_offset,
                                &mut y_offset,
                                row_color_fg_1,
                                row_color_bg_1,
                                row_color_fg_2,
                                row_color_bg_2,
                                &mut row_color_toggle,
                                f,
                            );
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

fn render_row(
    text: String,
    layout: Rect,
    x_offset: u16,
    y_offset: &mut u16,
    row_color_fg_1: Color,
    row_color_bg_1: Color,
    row_color_fg_2: Color,
    row_color_bg_2: Color,
    row_color_toggle: &mut bool,
    f: &mut Frame,
) {
    *y_offset += 1;
    *row_color_toggle = !*row_color_toggle;
    let mut p = Paragraph::new(text);
    if *row_color_toggle {
        p = p.set_style(Style::default().fg(row_color_fg_1).bg(row_color_bg_1));
    } else {
        p = p.set_style(Style::default().fg(row_color_fg_2).bg(row_color_bg_2));
    }
    let mut layout = layout;
    layout.x += x_offset;
    layout.y += *y_offset;
    layout.width -= 3;
    layout.height = 1;
    f.render_widget(p, layout);
}
