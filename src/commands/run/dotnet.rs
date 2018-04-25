// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::fs;
use std::io::{Error, ErrorKind};
use std::process;
use std::str;
use std::env;
use std::path::PathBuf;
use utils::*;
use semver::Version;

use super::Config;

pub fn install(config: &Config) -> Result<(), Error> {

    if !should_install(config) {
        return Ok(());
    }

    // The wanted SDK version already installed?
    let sdk_version = match Version::parse(&config.sdk_version.as_ref().unwrap()[..]) {
        Ok(v) => v,
        Err(_) => return Err(Error::new(ErrorKind::Other, "SDK version is not valid."))
    };

    // Get the currently installed version.
    let installed_version = get_installed_version()?;
    if installed_version >= sdk_version {
        // Newer version installed.
        println!("Dotnet SDK {} is already installed (wanted {}).", &installed_version, &sdk_version);
        return Ok(());
    } else if installed_version == sdk_version {
        // Exact version installed.
        println!("Dotnet SDK {} is already installed.", &sdk_version);
        return Ok(());
    }

    // Make sure that the .dotnet directory exists.
    let dotnet_path = config.root.join(".dotnet");
    if !dotnet_path.exists() {
        fs::create_dir(&dotnet_path)?;
    }
    // Make sure the platform directory exists.
    let platform = platform::get_platform_name()?;
    let dotnet_path = dotnet_path.join(platform);
    if !dotnet_path.exists() {
        fs::create_dir(&dotnet_path)?;
    }

    // Execute the installation script.
    execute_install_script(&dotnet_path, &sdk_version)?;

    // Set environment variables.
    set_environment_variables(&dotnet_path);

    // Get the installed version again.
    println!("Verifying installation...");
    let installed_version = get_installed_version()?;
    if installed_version < sdk_version {
        return Err(Error::new(ErrorKind::Other, "It looks like dotnet wasn't properly installed."));
    } else {
        println!("Dotnet SDK {} has been installed.", &installed_version);
    }

    return Ok(());
}

pub fn should_install(config: &Config) -> bool {
    return config.sdk_version.is_some();
}

fn get_installed_version() -> Result<Version, Error> {
    // Get the currently installed dotnet version.
    let output = process::Command::new("dotnet").arg("--version").output()?;
    if !output.status.success() {
        return Ok(Version::parse("0.0.0").unwrap());
    }

    // Same as the wanted version?
    let version = str::from_utf8(&output.stdout).unwrap().trim();
    return Ok(Version::parse(version).unwrap());
}

fn set_environment_variables(dotnet_path: &PathBuf) {
    // Update the environment path.
    let env_path = &env::var("PATH").unwrap()[..];
    env::set_var("PATH", format!("{};{}", dotnet_path.display(), env_path));

    // Add the installation directory as the first path.
    env::set_var("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1");
    env::set_var("DOTNET_CLI_TELEMETRY_OPTOUT", "1");
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn execute_install_script(dotnet_path: &PathBuf, version: &str) -> Result<(), Error> {
    // Download the installation script.
    let dotnet_script = dotnet_path.join("dotnet-install.sh");
    let dotnet_url = String::from("https://dot.net/v1/dotnet-install.sh");
    println!("Downloading https://dot.net/v1/dotnet-install.sh...");
    http::download(&dotnet_url, &dotnet_script, None)?;

    // Give the script executable permissions.
    process::Command::new("chmod")
                .arg("+x").arg(version)
                .arg(&dotnet_script)
                .output()?;

    // Execute the script.
    println!("Installing .NET Core SDK...");
    process::Command::new(&dotnet_script)
                .arg("--version").arg(version)
                .arg("--install-dir").arg(&dotnet_path)
                .arg("--no-path")
                .output()?;

    return Ok(());
}

#[cfg(target_os = "windows")]
fn execute_install_script(dotnet_path: &PathBuf, version: &Version) -> Result<(), Error> {
    // Download the installation script.
    let dotnet_script = dotnet_path.join("dotnet-install.ps1");
    let dotnet_url = String::from("https://dot.net/v1/dotnet-install.ps1");
    println!("Downloading https://dot.net/v1/dotnet-install.ps1...");
    http::download(&dotnet_url, &dotnet_script, None)?;

    // Convert the version to a string.
    let version = format!("{}.{}.{}", version.major, version.minor, version.patch);

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