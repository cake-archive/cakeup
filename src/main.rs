// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

extern crate clap;
extern crate semver;
#[macro_use]
extern crate serde_derive;

mod commands;
mod utils;

use clap::{App, Arg, SubCommand};
use commands::{Command, CommandType};
use std::process;

fn main() {
    let mut app = App::new("cakeup")
        .bin_name("cakeup")
        .about("A binary bootstrapper for Cake.")
        .version(utils::version::VERSION)
        .subcommand(SubCommand::with_name("run")
            .version(utils::version::VERSION)
            .about("Runs installation and execution of Cake and related tools")
            .arg(Arg::with_name("cake").takes_value(true).long("cake").help("The version of Cake to install."))
            .arg(Arg::with_name("nuget").takes_value(true).long("nuget").help("The version of NuGet to install."))
            .arg(Arg::with_name("sdk").takes_value(true).long("sdk").help("The version of the dotnet SDK to install."))
            .arg(Arg::with_name("execute").long("execute").help("Executes the Cake script."))
            .arg(Arg::with_name("bootstrap").long("bootstrap").help("Bootstraps Cake modules."))
            .arg(Arg::with_name("coreclr").long("coreclr").help("Use the CoreCLR version of Cake."))
            .arg(Arg::with_name("remaining").multiple(true).last(true)));

    // Run the command!
    let command = get_command(&mut app);
    command.run(app).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}

fn get_command(app: &mut App) -> Box<Command> {
    let args = app.clone().get_matches(); // get_matches take ownership.
    return match args.subcommand_name() {
        Some("run") => commands::create(CommandType::Run),
        _ => commands::create(CommandType::Help)
    };
}
