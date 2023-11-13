use crate::app::commands::command_errors;
use crate::app::App;
use crate::units::pressure;
use crate::units::temperature;

use super::command_errors::err_unit_usage;

pub fn run_command(app: &mut App, command_args: Vec<&str>) {
    let first_arg = command_args.get(0);
    if first_arg.is_none() {
        app.command_line_error = true;
        app.command_line.text = String::from("Unknown command!");
        return;
    }

    let first_arg = first_arg.unwrap();
    match *first_arg {
        ":units" => {
            let value = command_args.get(1);
            if value.is_some() {
                match value.unwrap().to_lowercase().as_str() {
                    "si" => {
                        app.gas_detail_view_state.set_units_si();
                        app.command_line.clear();
                        app.command_line_active = false;
                    }
                    "us" => {
                        app.gas_detail_view_state.set_units_us();
                        app.command_line.clear();
                        app.command_line_active = false;
                    }
                    _ => err_unit_usage(app),
                }
            } else {
                command_errors::err_unit_usage(app)
            }
        }

        ":p" => {
            let value = command_args[1].parse();
            if value.is_ok() {
                let value = value.unwrap();
                match command_args.get(2) {
                    Some(arg_2) => match arg_2.to_lowercase().as_str() {
                        "kpa" => {
                            let new_pressure =
                                pressure::Pressure::new(value, pressure::Unit::Kpa, true);
                            app.gas_detail_view_state.set_pressure_state(new_pressure);
                            app.command_line.clear();
                            app.command_line_active = false;
                        }
                        "pa" => {
                            let new_pressure =
                                pressure::Pressure::new(value, pressure::Unit::Pa, true);
                            app.gas_detail_view_state.set_pressure_state(new_pressure);
                            app.command_line.clear();
                            app.command_line_active = false;
                        }
                        "bar" => {
                            let new_pressure =
                                pressure::Pressure::new(value, pressure::Unit::Bar, true);
                            app.gas_detail_view_state.set_pressure_state(new_pressure);
                            app.command_line.clear();
                            app.command_line_active = false;
                        }
                        "psi" => {
                            let new_pressure =
                                pressure::Pressure::new(value, pressure::Unit::Psi, true);
                            app.gas_detail_view_state.set_pressure_state(new_pressure);
                            app.command_line.clear();
                            app.command_line_active = false;
                        }
                        "atm" => {
                            let new_pressure =
                                pressure::Pressure::new(value, pressure::Unit::Atm, true);
                            app.gas_detail_view_state.set_pressure_state(new_pressure);
                            app.command_line.clear();
                            app.command_line_active = false;
                        }
                        _ => command_errors::err_pressure_usage(app),
                    },
                    None => command_errors::err_pressure_usage(app),
                }
            }
        }

        ":t" => {
            let value = command_args[1].parse();
            if value.is_ok() {
                let value = value.unwrap();
                match command_args.get(2) {
                    Some(arg_2) => match arg_2.to_lowercase().as_str() {
                        "k" => {
                            let new_temperature =
                                temperature::Temperature::new(value, temperature::Unit::K);
                            app.gas_detail_view_state
                                .set_temperature_state(new_temperature);
                            app.command_line.clear();
                            app.command_line_active = false;
                        }
                        "c" => {
                            let new_temperature =
                                temperature::Temperature::new(value, temperature::Unit::C);
                            app.gas_detail_view_state
                                .set_temperature_state(new_temperature);
                            app.command_line.clear();
                            app.command_line_active = false;
                        }
                        "r" => {
                            let new_temperature =
                                temperature::Temperature::new(value, temperature::Unit::R);
                            app.gas_detail_view_state
                                .set_temperature_state(new_temperature);
                            app.command_line.clear();
                            app.command_line_active = false;
                        }
                        "f" => {
                            let new_temperature =
                                temperature::Temperature::new(value, temperature::Unit::F);
                            app.gas_detail_view_state
                                .set_temperature_state(new_temperature);
                            app.command_line.clear();
                            app.command_line_active = false;
                        }
                        _ => command_errors::err_temperature_usage(app),
                    },
                    None => command_errors::err_temperature_usage(app),
                }
            } else {
            }
        }
        _ => {
            app.command_line_error = true;
            app.command_line.text = String::from("Unknown command!");
        }
    }
}
