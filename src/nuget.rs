// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::process;

use utils::{http, CakeupResult};
use Config;

pub fn install(config: &Config) -> CakeupResult<()> {
    if should_install(config) {
        let file = config.tools.join("nuget.exe");
        if !file.exists() {
            let version = config.nuget_version.as_ref().unwrap();
            let url = format!(
                "https://dist.nuget.org/win-x86-commandline/{}/nuget.exe",
                version
            );
            info!("Downloading {}...", url);
            http::download(&url, &file, Some("Cakeup"))?;

            // Running on non-Windows platform?
            if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
                // Give the script executable permissions.
                process::Command::new("chmod")
                    .arg("+x")
                    .arg(version)
                    .arg(&file)
                    .output()?;
            }
        } else {
            info!("Nuget is already installed.");
        }
    }
    return Ok(());
}

pub fn should_install(config: &Config) -> bool {
    return match config.nuget_version {
        None => false,
        _ => true,
    };
}
