// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::fs;
use std::io::{Error, ErrorKind};
use std::process;
use std::str;
use std::env;
use std::path::PathBuf;
use config::*;
use utils::*;

pub fn install(config: &Config) -> Result<(), Error> {

    if !should_install(config) {
        return Ok(());
    }

    // Get the currently installed version.
    let installed_version = get_installed_version().unwrap_or_else(|_err| {
        return String::from("");
    });

    // The wanted SDK version already installed?
    let sdk_version = &config.sdk_version.as_ref().unwrap()[..];
    if installed_version == sdk_version {
        return Ok(());
    }

    // Make sure that the .dotnet directory exists.
    let dotnet_path = config.root.join(".dotnet");
    if !dotnet_path.exists() {
        println!("Creating .dotnet directory...");
        fs::create_dir(&dotnet_path)?;
    }

    // Execute the installation script.
    if cfg!(unix) {
        execute_shell_script(&dotnet_path, &sdk_version)?;
    } else {
        execute_powershell_script(&dotnet_path, &sdk_version)?;
    }

    // Set environment variables.
    set_environment_variables(&dotnet_path);

    // Get the installed version again.
    println!("Verifying installation...");
    let installed_version = get_installed_version()?;
    if installed_version != sdk_version {
        return Err(Error::new(ErrorKind::Other, "It looks like dotnet wasn't properly installed."));
    }
    println!("Installation OK.");

    return Ok(());
}

fn get_installed_version() -> Result<String, Error> {
    // Get the currently installed dotnet version.
    let output = process::Command::new("dotnet")
                .arg("--version").output()?;

    if !output.status.success() {
        return Err(Error::new(ErrorKind::Other, "Could not get installed version."));
    }

    // Same as the wanted version?
    let version = str::from_utf8(&output.stdout).unwrap().trim();
    return Ok(String::from(version));
}

fn set_environment_variables(dotnet_path: &PathBuf) {
    // Update the environment path.
    let env_path = &env::var("PATH").unwrap()[..];
    env::set_var("PATH", format!("{};{}", dotnet_path.display(), env_path));

    // Add the installation directory as the first path.
    env::set_var("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1");
    env::set_var("DOTNET_CLI_TELEMETRY_OPTOUT", "1");
}

fn execute_shell_script(dotnet_path: &PathBuf, version: &str) -> Result<(), Error> {
    // Download the installation script.
    let dotnet_script = dotnet_path.join("dotnet-install.sh");
    let dotnet_url = String::from("https://dot.net/v1/dotnet-install.sh");
    println!("Downloading .NET Core SDK installation script...");
    http::download(&dotnet_url, &dotnet_script, None)?;

    // Execute the script.
    println!("Installing .NET Core SDK...");
    process::Command::new(&dotnet_script)
                .arg("--version").arg(version)
                .arg("--install-dir").arg(&dotnet_path)
                .arg("--no-path")
                .output()?;

    return Ok(());
}

fn execute_powershell_script(dotnet_path: &PathBuf, version: &str) -> Result<(), Error> {
    // Download the installation script.
    let dotnet_script = dotnet_path.join("dotnet-install.ps1");
    let dotnet_url = String::from("https://dot.net/v1/dotnet-install.ps1");
    println!("Downloading .NET Core SDK installation script...");
    http::download(&dotnet_url, &dotnet_script, None)?;

    // Execute the script.
    println!("Installing .NET Core SDK...");
    process::Command::new("powershell")
                .arg("-NoProfile")
                .arg("-File").arg(dotnet_script)
                .arg("-Channel").arg("current")
                .arg("-Version").arg(version)
                .arg("-InstallDir").arg(&dotnet_path)
                .output()?;

    return Ok(());
}

fn should_install(config: &Config) -> bool {
    return match config.sdk_version {
        None => false,
        _ => true
    }
}