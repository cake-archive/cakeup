// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use clap::{App};
use utils::CakeupResult;

mod help;
mod run;

pub enum CommandType {
    Run,
    Help
}

pub trait Command {
    fn run(&self, app: App) -> CakeupResult<i32>;
}

pub fn create(command: CommandType) -> Box<Command> {
    return match command {
        CommandType::Run => Box::new(run::RunCommand { }),
        CommandType::Help => Box::new(help::HelpCommand {})
    }
}
