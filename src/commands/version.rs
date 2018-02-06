// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io;
use config::*;
use commands::Command;

// Embed the version number.
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct VersionCommand { }
impl Command for VersionCommand {
    fn run(&self, _config: &Config) -> Result<(), io::Error> {
        println!("{}", VERSION);
        return Ok(());
    }
}