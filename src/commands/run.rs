// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::fs;
use std::io::{Error,ErrorKind};
use config::*;
use commands::*;
use runners::*;

pub struct RunCommand { }
impl Command for RunCommand {
    fn run(&self, config: &Config) -> Result<(), Error> {

        // Create the tools directory.
        match create_tools_directory(&config) {
            Err(e) => return Err(Error::new(ErrorKind::Other, 
                format!("An error occured while creating the tools folder. {}", e))),
            _ => {}
        };

        // Download NuGet.
        match nuget::install(&config) {
            Err(e) => return Err(Error::new(ErrorKind::Other, 
                format!("An error occured while installing NuGet. {}", e))),
            _ => {}
        };

        // Download Cake.
        let cake = match cake::install(&config) {
            Ok(c) => c,
            Err(e) => return Err(Error::new(ErrorKind::Other, 
                format!("An error occured while downloading Cake. {}", e)))
        };

        // Install dotnet.
        match dotnet::install(&config) {
            Err(e) => return Err(Error::new(ErrorKind::Other, 
                format!("An error occured while installing dotnet. {}", e))),
            _ => {}
        };

        // Bootstrap Cake?
        if config.bootstrap {
            match cake.bootstrap(&config) {
                Err(e) => return Err(Error::new(ErrorKind::Other, 
                    format!("An error occured while bootstrapping Cake script. {}", e))),
                _ => {}
            };
        }

        // Execute Cake script.
        match cake.run(&config) {
            Err(e) => return Err(Error::new(ErrorKind::Other, 
                format!("An error occured while running Cake script. {}", e))),
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