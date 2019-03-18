use clap::App;
use cakeup::CakeupResult;

use crate::logger::Logger;

mod help;
mod run;

pub trait Command {
    fn run(&self, app: App) -> CakeupResult<i32>;
}

pub fn get_command(app: &mut App) -> Box<Command> {
    let args = app.clone().get_matches(); // get_matches take ownership.

    // Initialize logging.
    let trace = args.is_present("trace");
    Logger::init(trace).unwrap();

    // Create the right command.
    return match args.subcommand_name() {
        Some("run") => Box::new(run::RunCommand {}),
        _ => Box::new(help::HelpCommand {}),
    };
}