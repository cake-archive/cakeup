// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::fs;
use std::io::{Error,ErrorKind};
use self::config::Config;
use commands::*;

mod cake;
mod config;
mod dotnet;
mod nuget;

pub struct RunCommand {}
impl Command for RunCommand {
    fn run(&self, app: App) -> Result<(), Error> {

        // Parse configuration from arguments.
        let config = Config::parse(app);
        let mut executed_commands = 0;

        // Create the tools directory.
        if config.should_create_tools_directory() {
            match create_tools_directory(&config) {
                Err(e) => return Err(Error::new(ErrorKind::Other,
                    format!("An error occured while creating the tools folder. {}", e))),
                _ => { executed_commands += 1 }
            };
        }

        // NuGet
        if nuget::should_install(&config) {
            match nuget::install(&config) {
                Err(e) => return Err(Error::new(ErrorKind::Other,
                    format!("An error occured while installing NuGet. {}", e))),
                _ => { executed_commands += 1 }
            };
        }

        // .NET Core SDK
        if dotnet::should_install(&config) {
            match dotnet::install(&config) {
                Err(e) => return Err(Error::new(ErrorKind::Other,
                    format!("An error occured while installing dotnet. {}", e))),
                _ => { executed_commands += 1 }
            };
        }

        // Install Cake.
        if cake::should_install(&config) {
            let cake = match cake::install(&config) {
                Ok(c) => c,
                Err(e) => return Err(Error::new(ErrorKind::Other,
                    format!("An error occured while downloading Cake. {}", e)))
            };

            // Increase the number of executed commands.
            executed_commands += 1;

            // Was Cake installed?
            if cake.is_some() {
                let cake = cake.unwrap();

                // Bootstrap Cake?
                if config.bootstrap {
                    match cake.bootstrap(&config) {
                        Err(e) => return Err(Error::new(ErrorKind::Other,
                            format!("An error occured while bootstrapping Cake script. {}", e))),
                        _ => { executed_commands += 1 }
                    };
                }

                // Execute Cake script?
                if config.execute_script {
                    match cake.execute(&config) {
                        Err(e) => return Err(Error::new(ErrorKind::Other,
                            format!("An error occured while running Cake script. {}", e))),
                        _ => { executed_commands += 1 }
                    };
                }
            }
        }

        // Nothing was done?
        // Tell the user to avoid confusion.
        if executed_commands == 0 {
            if config.verbose {
                println!("No action was performed.");
            }
        }

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