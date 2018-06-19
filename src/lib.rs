#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate semver;

mod utils;

pub use config::Config;
pub use utils::CakeupResult;
pub use utils::version::VERSION;

use std::fs;

mod cake;
mod config;
mod dotnet;
mod host;
mod nuget;

pub fn run(config: Config) -> CakeupResult<i32> {
    // Create the tools directory.
    if config.should_create_tools_directory() {
        match create_tools_directory(&config) {
            Ok(()) => { }
            Err(e) => {
                return Err(format_err!(
                    "An error occured while creating the tools folder. {}",
                    e
                ))
            }
        };
    }

    // NuGet
    if nuget::should_install(&config) {
        match nuget::install(&config) {
            Ok(()) => { },
            Err(e) => {
                return Err(format_err!(
                    "An error occured while installing NuGet. {}",
                    e
                ))
            }
        };
    }

    // .NET Core SDK
    if dotnet::should_install(&config) {
        match dotnet::install(&config) {
            Ok(()) => { },
            Err(e) => {
                return Err(format_err!(
                    "An error occured while installing dotnet. {}",
                    e
                ))
            }
        };
    }

    // Install Cake.
    let mut result_code = 0;
    if cake::should_install(&config) {
        let cake = match cake::install(&config) {
            Ok(cake) => cake,
            Err(e) => {
                return Err(format_err!(
                    "An error occured while downloading Cake. {}",
                    e
                ))
            }
        };

        // Was Cake installed?
        if cake.is_some() {
            let cake = cake.unwrap();

            // Bootstrap Cake?
            if config.bootstrap {
                match cake.bootstrap(&config) {
                    Ok(_) => { },
                    Err(e) => {
                        return Err(format_err!(
                            "An error occured while bootstrapping Cake script. {}",
                            e
                        ))
                    }
                };
            }

            // Execute Cake script?
            if config.execute_script {
                match cake.execute(&config) {
                    Ok(n) => {
                        result_code = n;
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

    return Ok(result_code);
}

fn create_tools_directory(config: &Config) -> CakeupResult<()> {
    if !config.tools.exists() {
        info!("Creating tools directory...");
        fs::create_dir(&config.tools.to_str().unwrap())?;
    }
    return Ok(());
}
