// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::fs;
use std::io::{Error,ErrorKind};
use config::*;
use commands::Command;
use utils::*;

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
        match download_nuget(&config) {
            Err(e) => return Err(Error::new(ErrorKind::Other, 
                format!("An error occured while installing NuGet. {}", e))),
            _ => {}
        };

        // Download Cake.
        match download_cake(&config) {
            Err(e) => return Err(Error::new(ErrorKind::Other, 
                format!("An error occured while downloading Cake. {}", e))),
            _ => {}
        };

        // TODO: Bootstrap Cake?
        // TODO: Execute Cake script.

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

fn download_nuget(config: &Config) -> Result<(), Error> {
    if config.should_install_nuget() {
        let file = config.tools.join("nuget.exe");
        if !file.exists() {
            let version = config.nuget_version.as_ref().unwrap();
            let url = format!("https://dist.nuget.org/win-x86-commandline/{}/nuget.exe", version);
            println!("Downloading nuget ({})...", version);
            http::download(&url, &file)?;
        }
    }
    return Ok(());
}

fn download_cake(config: &Config) -> Result<(), Error> {

    // Get the version we're going to use.
    let mut version = config.cake_version.clone();
    if version == "latest" {
        println!("Asking GitHub what the latest release is...");
        let release = github::get_latest_release("cake-build", "cake")?;
        version = release.name;
    }

    // Install Cake.
    let cake_folder_path = config.tools.join(format!("cake.{}", &version));
    let cake_zip_filename = config.get_cake_filename(&version);
    let cake_zip_path = config.tools.join(&cake_zip_filename);
    if !cake_folder_path.exists() {

        // Zip file not present?
        if !cake_zip_path.exists() {
            println!("Downloading Cake {0}...", version);
            let url = &format!("https://github.com/cake-build/cake/releases/download/{0}/{1}", version, cake_zip_filename);
            http::download(&url, &cake_zip_path)?;
        }

        // Unzip Cake.
        println!("Unzipping binaries...");
        zip::unzip(&cake_zip_path, &cake_folder_path)?;
    }

    return Ok(());
}
