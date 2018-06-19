#[macro_use]
extern crate log;
extern crate cakeup;
extern crate clap;

mod commands;
mod utils;

use std::process;
use clap::{App, Arg, SubCommand};

pub fn main() {
    // Define arguments.
    let mut app = App::new("cakeup")
        .bin_name("cakeup")
        .about("A binary bootstrapper for Cake.")
        .version(cakeup::VERSION)
        .arg(
            Arg::with_name("trace")
                .short("t")
                .long("trace")
                .help("Show trace information."),
        )
        .subcommand(
            SubCommand::with_name("run")
                .version(cakeup::VERSION)
                .about("Runs installation and execution of Cake and related tools")
                .arg(
                    Arg::with_name("cake")
                        .takes_value(true)
                        .long("cake")
                        .help("The version of Cake to install."),
                )
                .arg(
                    Arg::with_name("nuget")
                        .takes_value(true)
                        .long("nuget")
                        .help("The version of NuGet to install."),
                )
                .arg(
                    Arg::with_name("sdk")
                        .takes_value(true)
                        .long("sdk")
                        .help("The version of the .NET Core SDK to install."),
                )
                .arg(
                    Arg::with_name("execute")
                        .long("execute")
                        .help("Executes the Cake script."),
                )
                .arg(
                    Arg::with_name("bootstrap")
                        .long("bootstrap")
                        .help("Bootstraps Cake modules."),
                )
                .arg(
                    Arg::with_name("coreclr")
                        .long("coreclr")
                        .help("Use the CoreCLR version of Cake."),
                )
                .arg(Arg::with_name("remaining").multiple(true).last(true)),
        );

    // Run the command!
    let command = commands::get_command(&mut app);
    let exit_code = command.run(app).unwrap_or_else(|err| {
        error!("{}", err);
        return -1;
    });

    process::exit(exit_code);
}
