// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use clap::App;
use commands::Command;
use utils::CakeupResult;

pub struct HelpCommand {}
impl Command for HelpCommand {
    fn run(&self, mut app: App) -> CakeupResult<i32> {
        if let &Err(ref e) = &app.print_long_help() {
            panic!("Could not print help: {}", e)
        };
        return Ok(0);
    }
}
