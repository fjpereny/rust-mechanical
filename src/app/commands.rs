use crate::app::ui::views::View;
use crate::app::App;

pub mod command_errors;
pub mod gas_details_view_commands;

pub fn run_command(app: &mut App, command: String) {
    let command_args: Vec<&str> = command.split(' ').collect();
    let first_arg = command_args.first();

    // Core application commands
    match first_arg {
        Some(arg) => match *arg {
            ":q!" => {
                app.should_quit = true;
                app.command_line.clear();
                app.command_line_active = false;
            }
            _ => (),
        },
        None => return,
    }

    // View specific commands
    match app.current_view {
        View::GasDetailView => gas_details_view_commands::run_command(app, command_args),
    }
}
