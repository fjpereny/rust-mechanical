use crate::app::App;

pub fn print_pressure_usage(app: &mut App) {
    app.command_line_error = true;
    app.command_line.text = String::from("Usage :p <pressure> <unit> example:[:p 100 kpa]");
}

pub fn print_temperature_usage(app: &mut App) {
    app.command_line_error = true;
    app.command_line.text = String::from("Usage :t <temperature> <unit> example:[:t 100 C]");
}
