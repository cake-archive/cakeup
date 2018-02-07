// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

#[macro_use]
extern crate serde_derive;

mod args;
mod config;
mod commands;
mod runners;
mod utils;

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
    if args.help {
        return commands::help();
    }
    if args.version {
        return commands::version();
    }
    return commands::run();
}
