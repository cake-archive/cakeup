// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::str;

use failure;
use semver::Version;

use Config;
use utils::CakeupResult;
use utils::{http, platform};

pub fn install(config: &Config) -> CakeupResult<()> {
    if !should_install(config) {
        return Ok(());
    }

    // Parse the wanted SDK version.
    let sdk_version = match Version::parse(&config.sdk_version.as_ref().unwrap()[..]) {
        Ok(v) => v,
        Err(_) => {
            return Err(failure::err_msg(
                "Provided .NET Core SDK version is not valid.",
            ))
        }
    };

    // Check the currently installed global version.
    let installed_version = get_installed_version(Option::None)?;
    if installed_version >= sdk_version {
        // Newer version installed.
        info!(
            ".NET Core SDK v{} is already installed globally (wanted v{}).",
            &installed_version, &sdk_version
        );
        return Ok(());
    }

    // Check the currently installed local version.
    let dotnet_path = get_local_installation_path(config)?;
    let installed_version = get_installed_version(Option::Some(&dotnet_path))?;
    if installed_version >= sdk_version {
        // Newer version installed.
        set_environment_variables(&dotnet_path)?;
        info!(
            ".NET Core SDK v{} is already installed locally (wanted v{}).",
            &installed_version, &sdk_version
        );
        return Ok(());
    }

    // Make sure that the install directory exists.
    let dotnet_path = create_install_directory(&config)?;

    // Execute the installation script.
    execute_install_script(&dotnet_path, &sdk_version)?;
    set_environment_variables(&dotnet_path)?;

    // Get the installed version again and verify that it's reachable.
    info!("Verifying installation...");
    let installed_version = get_installed_version(Option::None)?;
    if installed_version < sdk_version {
        return Err(format_err!(
            "Found v{} on PATH but wanted v{}.",
            &installed_version,
            &sdk_version
        ));
    } else {
        info!("Dotnet SDK v{} has been installed.", &installed_version);
    }

    return Ok(());
}

pub fn should_install(config: &Config) -> bool {
    return config.sdk_version.is_some();
}

fn get_local_installation_path(config: &Config) -> CakeupResult<PathBuf> {
    let platform = platform::get_platform_name()?;
    let path = config.root.join(".dotnet").join(platform);
    return Ok(path);
}

fn create_install_directory(config: &Config) -> CakeupResult<PathBuf> {
    let dotnet_path = config.root.join(".dotnet");
    if !dotnet_path.exists() {
        fs::create_dir(&dotnet_path)?;
    }
    let platform = platform::get_platform_name()?;
    let dotnet_path = dotnet_path.join(platform);
    if !dotnet_path.exists() {
        fs::create_dir(&dotnet_path)?;
    }
    return Ok(dotnet_path);
}

fn get_installed_version(path: Option<&PathBuf>) -> CakeupResult<Version> {
    let mut command = match path {
        None => process::Command::new("dotnet"),
        Some(path) => process::Command::new(path.join("dotnet")),
    };

    // Get the currently installed dotnet version.
    command.arg("--version");
    let version = match execute_and_return_output(&mut command) {
        Some(v) => v,
        None => String::from("0.0.0"),
    };

    return Ok(Version::parse(&version[..]).unwrap());
}

fn execute_and_return_output(command: &mut process::Command) -> Option<String> {
    match command.output() {
        Ok(result) => {
            if !result.status.success() {
                return Option::None;
            }
            return match str::from_utf8(&result.stdout) {
                Ok(output) => Option::Some(String::from(output)),
                Err(_) => Option::None,
            };
        }
        Err(_) => return Option::None,
    };
}

fn set_environment_variables(dotnet_path: &PathBuf) -> CakeupResult<()> {
    // Update the environment path.
    let env_path = &env::var("PATH").unwrap()[..];
    env::set_var(
        "PATH",
        format!(
            "{}{}{}",
            dotnet_path.display(),
            get_path_separator(),
            env_path
        ),
    );

    // Add the installation directory as the first path.
    env::set_var("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1");
    env::set_var("DOTNET_CLI_TELEMETRY_OPTOUT", "1");

    return Ok(());
}

fn get_path_separator() -> String {
    if cfg!(unix) {
        return String::from(":");
    }
    return String::from(";");
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn execute_install_script(dotnet_path: &PathBuf, version: &Version) -> CakeupResult<()> {
    // Download the installation script.
    let dotnet_script = dotnet_path.join("dotnet-install.sh");
    let dotnet_url = String::from("https://dot.net/v1/dotnet-install.sh");
    info!("Downloading https://dot.net/v1/dotnet-install.sh...");
    http::download(&dotnet_url, &dotnet_script, None)?;

    // Give the script executable permissions.
    process::Command::new("chmod")
        .arg("+x")
        .arg(&dotnet_script)
        .output()?;

    // Convert the version to a string.
    let version = format!("{}.{}.{}", version.major, version.minor, version.patch);

    // Execute the script.
    info!("Installing .NET Core SDK v{}...", version);
    process::Command::new(&dotnet_script)
        .arg("--version")
        .arg(version)
        .arg("--install-dir")
        .arg(&dotnet_path)
        .arg("--no-path")
        .output()?;

    return Ok(());
}

#[cfg(target_os = "windows")]
fn execute_install_script(dotnet_path: &PathBuf, version: &Version) -> CakeupResult<()> {
    // Download the installation script.
    let dotnet_script = dotnet_path.join("dotnet-install.ps1");
    let dotnet_url = String::from("https://dot.net/v1/dotnet-install.ps1");
    info!("Downloading https://dot.net/v1/dotnet-install.ps1...");
    http::download(&dotnet_url, &dotnet_script, None)?;

    // Convert the version to a string.
    let version = format!("{}.{}.{}", version.major, version.minor, version.patch);

    // Execute the script.
    info!("Installing .NET Core SDK v{}...", version);
    process::Command::new("powershell")
        .arg("-NoProfile")
        .arg("-File")
        .arg(dotnet_script)
        .arg("-Channel")
        .arg("current")
        .arg("-Version")
        .arg(version)
        .arg("-InstallDir")
        .arg(&dotnet_path)
        .output()?;

    return Ok(());
}
