// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io;
use ::config::Config;

mod help;
mod version;
mod install;

pub trait Command {
    fn run(&self, _config: &Config) -> Result<(), io::Error>;
}

pub fn help() -> Box<Command> {
    return Box::new(help::HelpCommand { })
}

pub fn version() -> Box<Command> {
    return Box::new(version::VersionCommand { })
}

pub fn install() -> Box<Command> {
    return Box::new(install::InstallCommand { })
}