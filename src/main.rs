// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

mod args;
mod config;
mod tasks;

use std::process;

fn main() {
    // Parse the configuration.
    let config = args::parse();

    // Get and execute all tasks.
    let tasks = tasks::get_tasks();
    for task in tasks {
        task.run(&config).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            process::exit(1);
        });
    }
}
