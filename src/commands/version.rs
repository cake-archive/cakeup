// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io;
use config::*;
use commands::Command;
use utils;

pub struct VersionCommand { }
impl Command for VersionCommand {
    fn run(&self, _config: &Config) -> Result<(), io::Error> {
        println!("{}", utils::version::VERSION);
        return Ok(());
    }
}