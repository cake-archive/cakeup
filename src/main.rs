// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

mod args;
mod config;
mod commands;

use std::process;
use args::*;
use commands::Command;

fn main() {
    // Parse arguments.
    let args = args::parse().unwrap_or_else(|err| {
        eprintln!("{}", err);
        println!("Use argument --help to see usage.");
        process::exit(1);
    });

    // Create the configuration.
    let config = config::Config::new(&args);

    // Run the appropriate command.
    let command = get_command(&args);
    command.run(&config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}

fn get_command(args: &Arguments) -> Box<Command> {
    if args.show_help {
        return commands::help();
    }
    if args.show_version {
        return commands::version();
    }
    return commands::install();
}
