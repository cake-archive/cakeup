// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::fs;
use std::io::{Error,ErrorKind};
use config::*;
use commands::Command;

pub struct InstallCommand { }
impl Command for InstallCommand {
    fn run(&self, config: &Config) -> Result<(), Error> {

        // Create the tools directory.
        match create_tools_directory(&config) {
            Err(e) => return Err(Error::new(ErrorKind::Other, 
                format!("An error occured while creating the tools folder. {}", e))),
            _ => {}
        };

        return Ok(());
    }
}

fn create_tools_directory(config: &Config) -> Result<(), Error> {
    if !config.tools.exists() {
        println!("Creating tools directory...");
        fs::create_dir(&config.tools.to_str().unwrap())?;
    }
    return Ok(());
}