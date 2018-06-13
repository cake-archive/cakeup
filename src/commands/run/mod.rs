// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use self::config::Config;
use commands::*;
use std::fs;
use utils::CakeupResult;

mod cake;
mod config;
mod dotnet;
mod host;
mod nuget;

pub struct RunCommand {}
impl Command for RunCommand {
    fn run(&self, app: App) -> CakeupResult<i32> {
        // Parse configuration from arguments.
        let config = Config::parse(app);
        let mut executed_commands = 0;

        // Create the tools directory.
        if config.should_create_tools_directory() {
            match create_tools_directory(&config) {
                Err(e) => {
                    return Err(format_err!(
                        "An error occured while creating the tools folder. {}",
                        e
                    ))
                }
                _ => executed_commands += 1,
            };
        }

        // NuGet
        if nuget::should_install(&config) {
            match nuget::install(&config) {
                Err(e) => {
                    return Err(format_err!(
                        "An error occured while installing NuGet. {}",
                        e
                    ))
                }
                _ => executed_commands += 1,
            };
        }

        // .NET Core SDK
        if dotnet::should_install(&config) {
            match dotnet::install(&config) {
                Err(e) => {
                    return Err(format_err!(
                        "An error occured while installing dotnet. {}",
                        e
                    ))
                }
                _ => executed_commands += 1,
            };
        }

        // Install Cake.
        let mut result_code = 0;
        if cake::should_install(&config) {
            let cake = match cake::install(&config) {
                Ok(c) => c,
                Err(e) => {
                    return Err(format_err!(
                        "An error occured while downloading Cake. {}",
                        e
                    ))
                }
            };

            // Increase the number of executed commands.
            executed_commands += 1;

            // Was Cake installed?
            if cake.is_some() {
                let cake = cake.unwrap();

                // Bootstrap Cake?
                if config.bootstrap {
                    match cake.bootstrap(&config) {
                        Err(e) => {
                            return Err(format_err!(
                                "An error occured while bootstrapping Cake script. {}",
                                e
                            ))
                        }
                        _ => executed_commands += 1,
                    };
                }

                // Execute Cake script?
                if config.execute_script {
                    match cake.execute(&config) {
                        Ok(n) => {
                            result_code = n;
                            executed_commands += 1;
                        }
                        Err(e) => {
                            return Err(format_err!(
                                "An error occured while executing Cake script. {}",
                                e
                            ))
                        }
                    };
                }
            }
        }

        // Nothing was done?
        // Tell the user to avoid confusion.
        if executed_commands == 0 {
            config.log.info(&format!("No action was performed."))?;
        }

        return Ok(result_code);
    }
}

fn create_tools_directory(config: &Config) -> CakeupResult<()> {
    if !config.tools.exists() {
        config.log.info("Creating tools directory...")?;
        fs::create_dir(&config.tools.to_str().unwrap())?;
    }
    return Ok(());
}
