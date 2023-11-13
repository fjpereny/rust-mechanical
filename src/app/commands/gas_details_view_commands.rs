use crate::app::App;
use crate::units::pressure;

pub fn run_command(app: &mut App, command_args: Vec<&str>) {
    let first_arg = command_args.get(0);
    if first_arg.is_none() {
        app.command_line_error = true;
        app.command_line.text = String::from("Unknown command!");
        return;
    }

    let first_arg = first_arg.unwrap();
    match *first_arg {
        ":p" => match command_args.get(2) {
            Some(val) => match val.to_lowercase().as_str() {
                "kpa" => {
                    let value: f32 = command_args[1].parse().unwrap();
                    let new_pressure = pressure::Pressure::new(value, pressure::Unit::Kpa, true);
                    app.gas_detail_view_state.set_pressure_state(new_pressure);
                    app.command_line.clear();
                    app.command_line_active = false;
                }
                "pa" => {
                    let value: f32 = command_args[1].parse().unwrap();
                    let new_pressure = pressure::Pressure::new(value, pressure::Unit::Pa, true);
                    app.gas_detail_view_state.set_pressure_state(new_pressure);
                    app.command_line.clear();
                    app.command_line_active = false;
                }
                "bar" => {
                    let value: f32 = command_args[1].parse().unwrap();
                    let new_pressure = pressure::Pressure::new(value, pressure::Unit::Bar, true);
                    app.gas_detail_view_state.set_pressure_state(new_pressure);
                    app.command_line.clear();
                    app.command_line_active = false;
                }
                "psi" => {
                    let value: f32 = command_args[1].parse().unwrap();
                    let new_pressure = pressure::Pressure::new(value, pressure::Unit::Psi, true);
                    app.gas_detail_view_state.set_pressure_state(new_pressure);
                    app.command_line.clear();
                    app.command_line_active = false;
                }
                "atm" => {
                    let value: f32 = command_args[1].parse().unwrap();
                    let new_pressure = pressure::Pressure::new(value, pressure::Unit::Atm, true);
                    app.gas_detail_view_state.set_pressure_state(new_pressure);
                    app.command_line.clear();
                    app.command_line_active = false;
                }
                _ => {
                    app.print_pressure_usage();
                }
            },
            None => {
                app.print_pressure_usage();
            }
        },
        _ => {
            app.command_line_error = true;
            app.command_line.text = String::from("Unknown command!");
        }
    }
}
