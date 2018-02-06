// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io;
use config::*;
use commands::Command;

pub struct InstallCommand { }
impl Command for InstallCommand {
    fn run(&self, config: &Config) -> Result<(), io::Error> {
        println!("{:#?}", config);
        return Ok(());
    }
}