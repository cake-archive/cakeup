// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io;
use clap::App;
use commands::Command;

pub struct HelpCommand {}
impl Command for HelpCommand {
    fn run(&self, mut app: App) -> Result<i32, io::Error> {
        if let &Err(ref e) = &app.print_long_help() {
            panic!("Could not print help: {}", e)
        };
        return Ok(0);
    }
}